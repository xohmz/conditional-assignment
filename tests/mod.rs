use conditional_assignment::ConditionalAssignment;

const TRUE_CHOICE: &str = "true";
const FALSE_CHOICE: &str = "false";

#[test]
fn pick_true() {
    assert_eq!(true.pick(TRUE_CHOICE, FALSE_CHOICE), TRUE_CHOICE)
}

#[test]
fn pick_false() {
    assert_eq!(false.pick(TRUE_CHOICE, FALSE_CHOICE), FALSE_CHOICE)
}

#[test]
fn ref_pick_true() {
    let some_bool = true;
    assert_eq!((&some_bool).pick(TRUE_CHOICE, FALSE_CHOICE), TRUE_CHOICE)
}

#[test]
fn ref_pick_false() {
    let some_bool = false;
    assert_eq!((&some_bool).pick(TRUE_CHOICE, FALSE_CHOICE), FALSE_CHOICE)
}

#[test]
fn pick_lazy_true() {
    let outcome = true.pick_lazy(
        || {
            assert!(true);
            TRUE_CHOICE
        },
        || {
            assert!(false);
            FALSE_CHOICE
        },
    );
    assert_eq!(outcome, TRUE_CHOICE)
}

#[test]
fn pick_lazy_false() {
    let outcome = false.pick_lazy(
        || {
            assert!(false);
            TRUE_CHOICE
        },
        || {
            assert!(true);
            FALSE_CHOICE
        },
    );
    assert_eq!(outcome, FALSE_CHOICE)
}

#[test]
fn ref_pick_lazy_true() {
    let some_bool = true;
    let outcome = (&some_bool).pick_lazy(
        || {
            assert!(true);
            TRUE_CHOICE
        },
        || {
            assert!(false);
            FALSE_CHOICE
        },
    );
    assert_eq!(outcome, TRUE_CHOICE)
}

#[test]
fn ref_pick_lazy_false() {
    let some_bool = false;
    let outcome = (&some_bool).pick_lazy(
        || {
            assert!(false);
            TRUE_CHOICE
        },
        || {
            assert!(true);
            FALSE_CHOICE
        },
    );
    assert_eq!(outcome, FALSE_CHOICE)
}
