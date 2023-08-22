use crate::parse;

#[test]
fn stage1() {
    assert_eq!(parse("1+1").unwrap(), 2.0);
    assert_eq!(parse("1 + 1").unwrap(), 2.0);
    assert_eq!(parse("1+2+3").unwrap(), 6.0);
    assert_eq!(parse("1-6").unwrap(), -5.0);
}

#[test]
fn stage2() {
    assert_eq!(parse("1*0").unwrap(), 0.0);
    assert_eq!(parse("2*3+1").unwrap(), 7.0);
    assert_eq!(parse("3+1*8").unwrap(), 11.0);
}

#[test]
fn stage3() {
    assert_eq!(parse("(1+1)").unwrap(), 2.0);
    assert_eq!(parse("(1)+(1)").unwrap(), 2.0);
    assert_eq!(parse("(3+1)*8").unwrap(), 32.0);
}

#[test]
fn stage4() {
    assert_eq!(parse("-1").unwrap(), -1.0);
    assert_eq!(parse("1+-1").unwrap(), 0.0);
    assert_eq!(parse("1--1").unwrap(), 2.0);
    assert_eq!(parse("1-(-(-1))").unwrap(), 0.0);
    assert_eq!(parse("(-5 + 2) * -1").unwrap(), 3.0);
}

#[test]
fn bonus() {
    assert!(parse("1++1").is_err());
    assert!(parse("(1+1").is_err());
    assert!(parse(")1+1").is_err());
}
