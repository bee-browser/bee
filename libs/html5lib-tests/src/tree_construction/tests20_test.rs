//<coverage:exclude>
use super::helper::parse;
use super::helper::Scripting;
use super::helper::Test;

#[test]
fn test_0000() {
    parse(Test {
        data: "<!doctype html><p><button><button>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (3, "<button>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0001() {
    parse(Test {
        data: "<!doctype html><p><button><address>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<address>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0002() {
    parse(Test {
        data: "<!doctype html><p><button><article>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<article>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0003() {
    parse(Test {
        data: "<!doctype html><p><button><aside>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<aside>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0004() {
    parse(Test {
        data: "<!doctype html><p><button><blockquote>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<blockquote>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0005() {
    parse(Test {
        data: "<!doctype html><p><button><center>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<center>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0006() {
    parse(Test {
        data: "<!doctype html><p><button><details>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<details>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0007() {
    parse(Test {
        data: "<!doctype html><p><button><dialog>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<dialog>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0008() {
    parse(Test {
        data: "<!doctype html><p><button><dir>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<dir>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0009() {
    parse(Test {
        data: "<!doctype html><p><button><div>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0010() {
    parse(Test {
        data: "<!doctype html><p><button><dl>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<dl>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0011() {
    parse(Test {
        data: "<!doctype html><p><button><fieldset>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<fieldset>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0012() {
    parse(Test {
        data: "<!doctype html><p><button><figcaption>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<figcaption>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0013() {
    parse(Test {
        data: "<!doctype html><p><button><figure>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<figure>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0014() {
    parse(Test {
        data: "<!doctype html><p><button><footer>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<footer>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0015() {
    parse(Test {
        data: "<!doctype html><p><button><header>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<header>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0016() {
    parse(Test {
        data: "<!doctype html><p><button><hgroup>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<hgroup>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0017() {
    parse(Test {
        data: "<!doctype html><p><button><main>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<main>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0018() {
    parse(Test {
        data: "<!doctype html><p><button><menu>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<menu>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0019() {
    parse(Test {
        data: "<!doctype html><p><button><nav>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<nav>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0020() {
    parse(Test {
        data: "<!doctype html><p><button><ol>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<ol>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0021() {
    parse(Test {
        data: "<!doctype html><p><button><p>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<p>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0022() {
    parse(Test {
        data: "<!doctype html><p><button><search>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<search>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0023() {
    parse(Test {
        data: "<!doctype html><p><button><section>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<section>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0024() {
    parse(Test {
        data: "<!doctype html><p><button><summary>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<summary>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0025() {
    parse(Test {
        data: "<!doctype html><p><button><ul>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<ul>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0026() {
    parse(Test {
        data: "<!doctype html><p><button><h1>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<h1>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0027() {
    parse(Test {
        data: "<!doctype html><p><button><h6>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<h6>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0028() {
    parse(Test {
        data: "<!doctype html><p><button><listing>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<listing>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0029() {
    parse(Test {
        data: "<!doctype html><p><button><pre>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<pre>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0030() {
    parse(Test {
        data: "<!doctype html><p><button><form>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<form>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0031() {
    parse(Test {
        data: "<!doctype html><p><button><li>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<li>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0032() {
    parse(Test {
        data: "<!doctype html><p><button><dd>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<dd>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0033() {
    parse(Test {
        data: "<!doctype html><p><button><dt>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<dt>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0034() {
    parse(Test {
        data: "<!doctype html><p><button><plaintext>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<plaintext>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0035() {
    parse(Test {
        data: "<!doctype html><p><button><table>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0036() {
    parse(Test {
        data: "<!doctype html><p><button><hr>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<hr>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0037() {
    parse(Test {
        data: "<!doctype html><p><button><xmp>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<xmp>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0038() {
    parse(Test {
        data: "<!doctype html><p><button></p>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<button>"),
            (4, "<p>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0039() {
    parse(Test {
        data: "<!doctype html><button><p></button>x",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<button>"),
            (3, "<p>"),
            (2, "\"x\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0040() {
    parse(Test {
        data: "<!doctype html><address><button></address>a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<address>"),
            (3, "<button>"),
            (2, "\"a\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0041() {
    parse(Test {
        data: "<!doctype html><address><button></address>a",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<address>"),
            (3, "<button>"),
            (2, "\"a\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0042() {
    parse(Test {
        data: "<p><table></p>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (3, "<p>"),
            (3, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0043() {
    parse(Test {
        data: "<!doctype html><svg>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0044() {
    parse(Test {
        data: "<!doctype html><p><figcaption>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<figcaption>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0045() {
    parse(Test {
        data: "<!doctype html><p><summary>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<p>"),
            (2, "<summary>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0046() {
    parse(Test {
        data: "<!doctype html><form><table><form>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<form>"),
            (3, "<table>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0047() {
    parse(Test {
        data: "<!doctype html><table><form><form>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<form>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0048() {
    parse(Test {
        data: "<!doctype html><table><form></table><form>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<table>"),
            (3, "<form>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0049() {
    parse(Test {
        data: "<!doctype html><svg><foreignObject><p>",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg foreignObject>"),
            (4, "<p>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0050() {
    parse(Test {
        data: "<!doctype html><svg><title>abc",
        document: vec![
            (0, "<!DOCTYPE html>"),
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<svg svg>"),
            (3, "<svg title>"),
            (4, "\"abc\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0051() {
    parse(Test {
        data: "<option><span><option>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<option>"),
            (3, "<span>"),
            (4, "<option>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0052() {
    parse(Test {
        data: "<option><option>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<option>"),
            (2, "<option>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0053() {
    parse(Test {
        data: "<math><annotation-xml><div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math annotation-xml>"),
            (2, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0054() {
    parse(Test {
        data: "<math><annotation-xml encoding=\"application/svg+xml\"><div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math annotation-xml>"),
            (4, "encoding=\"application/svg+xml\""),
            (2, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0055() {
    parse(Test {
        data: "<math><annotation-xml encoding=\"application/xhtml+xml\"><div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math annotation-xml>"),
            (4, "encoding=\"application/xhtml+xml\""),
            (4, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0056() {
    parse(Test {
        data: "<math><annotation-xml encoding=\"aPPlication/xhtmL+xMl\"><div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math annotation-xml>"),
            (4, "encoding=\"aPPlication/xhtmL+xMl\""),
            (4, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0057() {
    parse(Test {
        data: "<math><annotation-xml encoding=\"text/html\"><div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math annotation-xml>"),
            (4, "encoding=\"text/html\""),
            (4, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0058() {
    parse(Test {
        data: "<math><annotation-xml encoding=\"Text/htmL\"><div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math annotation-xml>"),
            (4, "encoding=\"Text/htmL\""),
            (4, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0059() {
    parse(Test {
        data: "<math><annotation-xml encoding=\" text/html \"><div>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math annotation-xml>"),
            (4, "encoding=\" text/html \""),
            (2, "<div>"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0060() {
    parse(Test {
        data: "<math><annotation-xml> </annotation-xml>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math annotation-xml>"),
            (4, "\" \""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0061() {
    parse(Test {
        data: "<math><annotation-xml>c</annotation-xml>",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math annotation-xml>"),
            (4, "\"c\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0062() {
    parse(Test {
        data: "<math><annotation-xml><!--foo-->",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math annotation-xml>"),
            (4, "<!-- foo -->"),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0063() {
    parse(Test {
        data: "<math><annotation-xml></svg>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math annotation-xml>"),
            (4, "\"x\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}

#[test]
fn test_0064() {
    parse(Test {
        data: "<math><annotation-xml><svg>x",
        document: vec![
            (0, "<html>"),
            (1, "<head>"),
            (1, "<body>"),
            (2, "<math math>"),
            (3, "<math annotation-xml>"),
            (4, "<svg svg>"),
            (5, "\"x\""),
        ],
        context_element: None,
        scripting: Scripting::Both,
    });
}
//</coverage:exclude>
