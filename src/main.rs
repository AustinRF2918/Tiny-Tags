enum XMLState
{
    Whitespace,
    Content(char),
    BeginTag,
    MiddleTag(char),
    EndTag,
}

enum XMLStateType
{
    Open,
    Close,
    Intermediate,
    None,
}

//Simple parse object for storing tokens.
struct ParseObject
{
    parseable_input : String,
}

fn iterate_state(state : &XMLState, letter : char) -> Option<XMLState>
{
    match (state, letter)
    {
        (&XMLState::Whitespace, '<') =>
        {
            Some(XMLState::BeginTag)
        },
        (&XMLState::BeginTag, '>') =>
        {
            Some(XMLState::EndTag)
        },
        (&XMLState::BeginTag, _) =>
        {
            Some(XMLState::MiddleTag(letter))
        },
        (&XMLState::MiddleTag(_), '>') =>
        {
            Some(XMLState::EndTag)
        },
        (&XMLState::MiddleTag(_), _) =>
        {
            Some(XMLState::MiddleTag(letter))
        },
        (&XMLState::EndTag, '<') =>
        {
            Some(XMLState::BeginTag)
        },
        (&XMLState::EndTag, _) =>
        {
            Some(XMLState::Content(letter))
        },
        (&XMLState::Content(_), '<') =>
        {
            Some(XMLState::BeginTag)
        },
        (&XMLState::Content(_), '>') =>
        {
            Some(XMLState::EndTag)
        },
        (&XMLState::Content(_), _) =>
        {
            Some(XMLState::Content(letter))
        },
        _ =>
        {
            None
        }
    }
}

//Logical backend.
fn execute_logic(stack: &Vec<String>, current: &String) -> Option<String>
{
    match stack.last()
    {
        Some(token) =>
        {
            match current.as_str()
            {
                "html" =>
                {
                    Some(token.clone())
                },
                "head" =>
                {
                    Some(token.clone())
                },
                "body" =>
                {
                    Some(token.clone())
                },
                _ =>
                {
                    Some(token.clone())
                },
            }
        }
        None =>
        {
            match current.as_str()
            {
                "html" => {Some("IS".to_string())},
                _ => {None}
            }
        }
    }
}

impl ParseObject
{
    pub fn new(input : &str) -> ParseObject
    {
        ParseObject{
            parseable_input : input.to_string(),
        }
    }

    pub fn parse(&mut self)
    {
        let mut state = XMLState::Whitespace;
        let mut string = String::new();
        let mut push_down : Vec<String> = Vec::new();
        let mut mode = XMLStateType::None;

        for i in self.parseable_input.chars()
        {
            match iterate_state(&state, i)
            {
                Some(new_state) => {
                    match new_state{
                        XMLState::MiddleTag(data) =>
                        {
                            match state{
                                XMLState::BeginTag =>
                                {
                                    if i == '/'
                                    {
                                        mode = XMLStateType::Close;
                                    }
                                    else if i == ' '
                                    {
                                        mode = XMLStateType::Intermediate;
                                    }
                                    else
                                    {
                                        string.push(data);
                                        mode = XMLStateType::Open;
                                    }
                                }
                                _ =>
                                {
                                    string.push(data);
                                }
                            }
                        },
                        XMLState::Content(data) =>
                        {
                            string.push(data);
                        },
                        XMLState::EndTag =>
                        {
                            match execute_logic(&push_down, &string)
                            {
                                Some(_) => {
                                    match mode
                                    {
                                        XMLStateType::Open =>
                                        {
                                            match string.pop()
                                            {
                                                Some('/') => {},
                                                Some(data) => {
                                                    let mut push = string.clone();
                                                    push.push(data);
                                                    push_down.push(push);
                                                },
                                                None => {
                                                    panic!("Bad syntax");
                                                }
                                            }
                                                
                                        }
                                        XMLStateType::Close =>
                                        {
                                            match push_down.last()
                                            {
                                                Some(tag) =>
                                                {
                                                    if &string == tag
                                                    {
                                                    }
                                                    else
                                                    {
                                                        panic!("Mismatched tags.{}", tag);
                                                    }

                                                }
                                                None =>
                                                {
                                                    panic!("Mismatched tags.");
                                                }
                                            }
                                                push_down.pop();
                                        }
                                        _ =>
                                        {
                                            panic!("Parsing error.");
                                        }
                                    }
                                }
                                None => {
                                    panic!("Parsing error, check for unclosed tags.");
                                }
                            }
                            string.clear();
                        }
                        XMLState::BeginTag =>
                        {
                            match (execute_logic(&push_down, &string), &mode)
                            {
                                (Some(state), &XMLStateType::Open) =>
                                {
                                    println!("{}", state);
                                    if string.as_str() != ""
                                    {
                                        match state.as_str()
                                        {
                                            //Put tag pushdown logic here.
                                            _  =>{
                                                println!("{}", string);
                                                string.clear();
                                            }
                                        }
                                    }
                                }
                                _ =>
                                {
                                }

                            }
                        }
                        _ => {}
                    }
                    state = new_state;
                }
                None => {
                    panic!("Error parsing.");
                }
            }
        }
    }
}
//End parse object.

fn main() {
    let em_file = "<html><head></head><body><h1>Hello World</h1></body></html><html><head></head><body><h1>Hello World</h1></body></html><html><head></head><body><h1>Hello World</h1></body></html><html><head></head><body><h1>Hello World</h1></body></html><html><head></head><body><h1>Hello World</h1></body></html><html><head></head><body><h1>Hello World</h1></body></html>";
    let mut parser = ParseObject::new(em_file);
    parser.parse();
}

