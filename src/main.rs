enum Datatypes{
    Bool,
    Array,
    String,
    Number,
    Object,
    Null
}

enum Token{
    CurlyOpen,
    CurlyClone,
    SquareOpen,
    SquareClose,
    Comma,
    Colon,
    String,
    Number,
    True,
    False,
    Null
}


struct Parse<'a>{
    input: &'a str,
    position: usize
}

impl<'a> Parse<'a>{
    fn new(json: &'a str) -> Parse<'a>{
        // Todo
        return Parse { input: json, position: 0 };
    }

    fn parse_value(){
        // Todo
    }

    fn eat(){
        // Todo
    }

    fn skip_whitespace(){
        // Todo
    }

    fn parse_array(){
        // Todo
    }

    fn parse_string(){
        // Todo
    }

    fn parse_object(){
        // Todo
    }

    fn parse_number(){
        // Todo
    }

}


fn main() {
    let json_str: &str = r#"
        {
            "name": "John",
            "age": 30,
            "is_student": false,
            "grades": [95, 88, 76],
            "address": {
                "city": "New York",
                "zip": "10001"
            }
        }
    "#;

    let mut parser = Parse::new(json_str);
    let parsed_json = parser.parse_value();
    if parsed_json{
        println!("{}", parsed_json);
    }
    else{
        println!{"Failed to open Json file"};
    }

}
