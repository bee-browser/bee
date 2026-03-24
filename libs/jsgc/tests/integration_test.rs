use jsgc::HandleMut;
use jsgc::Heap;
use jsgc::Unknown;
use jsgc::UnknownVtable;
use jsgc::VisitList;

#[derive(Default)]
struct Cell {
    car: Option<HandleMut<Cell>>,
    cdr: Option<HandleMut<Cell>>,
}

impl Cell {
    fn trace(&self, visit_list: &mut VisitList) {
        if let Some(car) = self.car {
            visit_list.push(car.as_addr());
        }
        if let Some(cdr) = self.cdr {
            visit_list.push(cdr.as_addr());
        }
    }
}

impl Unknown for Cell {
    fn vtable() -> &'static UnknownVtable {
        fn trace(addr: usize, visit_list: &mut VisitList) {
            HandleMut::<Cell>::from_addr(addr)
                .unwrap()
                .trace(visit_list);
        }

        static VTABLE: UnknownVtable = UnknownVtable {
            tidy: None,
            trace: Some(trace),
        };

        &VTABLE
    }
}

#[test]
fn test_collect_garbage() {
    let mut heap = Heap::new();

    macro_rules! cell {
        () => {
            heap.alloc_mut(Cell::default())
        };
    }

    macro_rules! cons {
        ($car:expr, $cdr:expr,) => {
            cons!($car, $cdr)
        };
        ($car:expr, $cdr:expr) => {{
            let car = Some($car);
            let cdr = Some($cdr);
            heap.alloc_mut(Cell { car, cdr })
        }};
    }

    macro_rules! gc {
        ($roots:expr) => {
            heap.collect_garbage(&$roots)
        };
    }

    macro_rules! num_objects {
        () => {
            heap.stats().num_objects
        };
    }

    let cell = cell!();
    assert_eq!(num_objects!(), 1);

    let tree = cons!(cons!(cell!(), cell!()), cons!(cell!(), cell!()));
    assert_eq!(num_objects!(), 8);

    let ring = {
        let mut start = cell!();
        let mut mid = cell!();
        let mut end = cell!();
        start.cdr = Some(mid);
        mid.cdr = Some(end);
        end.cdr = Some(start);
        start
    };
    assert_eq!(num_objects!(), 11);

    gc!([cell.as_addr(), tree.as_addr(), ring.as_addr()]);
    assert_eq!(num_objects!(), 11);

    gc!([tree.as_addr(), ring.as_addr()]);
    assert_eq!(num_objects!(), 10);

    gc!([ring.as_addr()]);
    assert_eq!(num_objects!(), 3);

    gc!([]);
    assert_eq!(num_objects!(), 0);
}

#[test]
fn test_unmanaged_tracing_targets() {
    let mut heap = Heap::new();

    let mut root = Cell::default(); // unmanaged
    let mut root = HandleMut::from_mut(&mut root);

    // TODO(feat): not ergonomic
    heap.add_tracee(root.into());

    assert_eq!(heap.stats().num_objects, 0);

    root.car = Some(heap.alloc_mut(Cell::default()));
    root.cdr = Some(heap.alloc_mut(Cell::default()));
    assert_eq!(heap.stats().num_objects, 2);

    heap.collect_garbage(&[root.as_addr()]);
    assert_eq!(heap.stats().num_objects, 2);

    // TODO(feat): not ergonomic
    heap.remove_tracee(root.into());

    heap.collect_garbage(&[]);
    assert_eq!(heap.stats().num_objects, 0);

    // TODO(feat): UAF... root.[car|cdr] are still accessible.
}
