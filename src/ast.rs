use crate::lexer::Token;

#[derive(Debug)]
pub enum Node
{
    Command(CommandNode),
    Scope(Vec<Node>)
}

#[derive(Debug)]
pub enum CommandNode
{
    Left(usize),
    Right(usize),
    Add(usize),
    Sub(usize),
    Read,
    Write(WriteFormat),
    CacheUp,
    CacheDown,
}

#[derive(Debug)]
pub enum WriteFormat
{
    Ascii,
    Decimal,
}

#[derive(Debug)]
pub enum SyntaxError
{
    MissingScopeCloseToken,
    MissingScopeOpenToken,
}

pub fn gen_ast(tokens: Vec<Token>) -> Result<Vec<Node>, SyntaxError>
{
    let mut scope_stack: Vec<Vec<Node>> = vec![Vec::new()];

    for token in tokens
    {
        if token == Token::ScopeOpen
        {
            scope_stack.push(Vec::new());
            continue;
        }
        else if token == Token::ScopeClose
        {
            if let Some(node) = scope_stack.pop()
            {
                if let Some(parent) = scope_stack.last_mut()
                {
                    parent.push(Node::Scope(node));
                    continue;
                }
                else
                {
                    return Err(SyntaxError::MissingScopeCloseToken);
                }
            }
            else
            {
                panic!();
            }
        }

        if let Some(mut parent) = scope_stack.last_mut()
        {
            let node = match token
            {
                Token::Left(t) =>       Node::Command(CommandNode::Left(t)),
                Token::Right(t) =>      Node::Command(CommandNode::Right(t)),
                Token::Add(t) =>        Node::Command(CommandNode::Add(t)),
                Token::Sub(t) =>        Node::Command(CommandNode::Sub(t)),
                Token::Read =>          Node::Command(CommandNode::Read),
                Token::CacheUp =>       Node::Command(CommandNode::CacheUp),
                Token::CacheDown =>     Node::Command(CommandNode::CacheDown),
                Token::WriteAscii =>    Node::Command(CommandNode::Write(WriteFormat::Ascii)),
                Token::WriteDec =>      Node::Command(CommandNode::Write(WriteFormat::Decimal)),
                _ => panic!(),
            };

            parent.push(node);
        }
        else
        {
            panic!();
        }
    }

    Ok(scope_stack.pop().unwrap())
}