use crate::style::*;

#[derive(Clone, Debug)]
pub(crate) struct Spec {
    pub(crate) node: NodeSpec,
    pub(crate) container: ContainerSpec,
}

impl Spec {
    pub(crate) fn determine_from(style: &Style) -> Self {
        Spec::default()
            .check_display(&style.display)
            //.check_overflow(style.overflow_x,  style.overflow_y)
            //.check_floating(style.floating)
            //.check_content(style)
            .check_positioning(style.positioning)
            .validate()
            .determine()
    }

    fn check_display<'a>(&'a mut self, display: &DisplayStyle) -> &'a mut Self {
        self.node = match display.outside {
            DisplayOutside::None => NodeSpec::None,
            DisplayOutside::Inline => NodeSpec::Inline,
            DisplayOutside::Block => NodeSpec::Block,
            _ => NodeSpec::None,
        };

        self.container = match display.inside {
            DisplayInside::None => ContainerSpec::None,
            DisplayInside::FlowRoot => ContainerSpec::Flow,
            DisplayInside::Flow => match display.outside {
                DisplayOutside::Block => ContainerSpec::Block,
                _ => ContainerSpec::Inline,
            },
            DisplayInside::Flex => ContainerSpec::Flex,
            DisplayInside::Canvas => ContainerSpec::Canvas,
            _ => ContainerSpec::None,
        };

        self
    }

    fn check_positioning<'a>(&'a mut self, positioning: PositioningScheme) -> &'a mut Self {
        if !self.node.is_none() {
            match positioning {
                PositioningScheme::Absolute
                | PositioningScheme::Fixed
                | PositioningScheme::Sticky => {
                    self.node = NodeSpec::Layer;
                    self.container = ContainerSpec::Flow;
                }
                _ => (),
            }
        }

        self
    }

    fn validate<'a>(&'a self) -> &'a Self {
        // TODO: panics in inconsistency
        self
    }

    fn determine(&self) -> Self {
        self.clone()
    }
}

impl Default for Spec {
    fn default() -> Self {
        Spec {
            node: NodeSpec::None,
            container: ContainerSpec::None,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub(crate) enum NodeSpec {
    None,
    Block,
    Inline,
    Float,
    Layer,
}

impl NodeSpec {
    #[inline]
    pub(crate) fn is_none(&self) -> bool {
        if let Self::None = self {
            true
        } else {
            false
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub(crate) enum ContainerSpec {
    None,
    Flow,
    Block,
    Inline,
    Flex,
    Canvas,
}
