use conditional_assignment::ConditionalAssignment;

const POS: &str = "positive";
const NEG: &str = "negative";

#[test]
fn bool_pick_true() {
    assert_eq!(true.pick(POS, NEG), POS)
}

#[test]
fn bool_pick_false() {
    assert_eq!(false.pick(POS, NEG), NEG)
}

#[test]
fn bool_ref_pick_true() {
    let some_bool = true;
    assert_eq!((&some_bool).pick(POS, NEG), POS)
}

#[test]
fn option_pick_some() {
    assert_eq!(Option::Some(()).pick(POS, NEG), POS)
}

#[test]
fn option_pick_none() {
    assert_eq!(Option::None::<()>.pick(POS, NEG), NEG)
}

#[test]
fn option_ref_pick_true() {
    let some_option = Option::Some(());
    assert_eq!((&some_option).pick(POS, NEG), POS)
}

#[test]
fn result_pick_ok() {
    assert_eq!(Result::Ok::<(), ()>(()).pick(POS, NEG), POS)
}

#[test]
fn result_pick_err() {
    assert_eq!(Result::Err::<(), ()>(()).pick(POS, NEG), NEG)
}

#[test]
fn result_ref_pick_true() {
    let some_result = Result::Ok::<(), ()>(());
    assert_eq!((&some_result).pick(POS, NEG), POS)
}

#[test]
fn bool_option_result_pick_positives() {
    assert_eq!(
        (1 == 1)
            .pick(Option::Some(()), Option::None::<()>)
            .pick(Result::Ok::<(), ()>(()), Result::Err::<(), ()>(()))
            .pick(POS, NEG),
        POS
    )
}

#[test]
fn bool_option_result_pick_negatives() {
    assert_eq!(
        (1 != 1)
            .pick(Option::Some(()), Option::None::<()>)
            .pick(Result::Ok::<(), ()>(()), Result::Err::<(), ()>(()))
            .pick(POS, NEG),
        NEG
    )
}

#[test]
fn bool_str_bool_positives() {
    assert_eq!(
        true.pick("positive", "negative")
            .starts_with("pos")
            .pick(POS, NEG),
        POS
    )
}

#[test]
fn str_manipulation_positive() {
    assert_eq!(
        " Conditional Assignment "
            .trim()
            .to_lowercase()
            .eq("conditional assignment")
            .pick(POS, NEG),
        POS
    )
}

#[test]
fn str_vec_manipulation() {
    let msg = "one two three"
        .split_whitespace()
        .next()
        .is_some_and(|s| s.len() == 3)
        .pick(
            "Found first word with three characters.",
            "Did not find first word with three characters.",
        );

    assert_eq!(msg, "Found first word with three characters.")
}

#[test]
fn check_laziness() {
    let mut positive_counter = 0;
    let mut negative_counter = 0;

    let say = true.pick_lazy(
        || {
            positive_counter += 1;
            "It was true!"
        },
        || {
            negative_counter += 1;
            "It was false!"
        },
    );

    assert_eq!(positive_counter, 1);
    assert_eq!(negative_counter, 0);
    assert_eq!(say, "It was true!");
}
