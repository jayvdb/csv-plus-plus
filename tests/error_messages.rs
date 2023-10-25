use csvpp::Template;
mod common;

#[test]
fn syntax_error_in_code_section() {
    let s = common::Setup::from_str(
        "syntax_error_in_code_section",
        "csv",
        r#"
## Welcome to the all_features.csvpp test. this is a comment
##
fn foo_fn<a, b, c> a + b * c
---
foo,bar
"#,
    );
    let template = Template::compile(&s.runtime);

    assert_eq!(
        template.unwrap_err().to_string(),
        "Syntax error in code section of integration_test_syntax_error_in_code_section.csvpp
On line 4:9, Expected `(` but saw `<`

 1: 
 2: ## Welcome to the all_features.csvpp test. this is a comment
 3: ##
 4: fn foo_fn<a, b, c> a + b * c
  : ---------^
 5: ---
 6: foo,bar

"
    );
}

#[test]
fn syntax_error_in_modifier_definition() {
    let s = common::Setup::from_str(
        "syntax_error_in_modifier_definition",
        "csv",
        r#"
---
foo,bar,[[format=bold ,foo
"#,
    );
    let template = Template::compile(&s.runtime);

    assert_eq!(
        template.unwrap_err().to_string(),
        "Invalid modifier definition in cell C1 (2, 0) of integration_test_syntax_error_in_modifier_definition.csvpp
On line 3:21, Error parsing input, expected ']]' but saw unrecognized token ``

 1: 
 2: ---
 3: foo,bar,[[format=bold ,foo
  : ---------------------^

"
    );
}

#[test]
fn bad_choice_in_modifier_with_possibilities() {
    let s = common::Setup::from_str(
        "bad_choice_in_modifier_with_possibilities",
        "csv",
        r#"
---
foo,bar,[[b=foo]],foo
"#,
    );
    let template = Template::compile(&s.runtime);

    assert_eq!(
        template.unwrap_err().to_string(),
        "Invalid modifier definition in cell C1 (2, 0) of integration_test_bad_choice_in_modifier_with_possibilities.csvpp
On line 3:15, received invalid value when parsing `border` modifier but saw `foo`
Possible values: all (a) | top (t) | bottom (b) | left (l) | right (r)

 1: 
 2: ---
 3: foo,bar,[[b=foo]],foo
  : ---------------^

"
    );
}
