struct Stack<T> {
     node:Vec<Box<T>>
}

trait Common<T> {
     fn length(&self)->i32;
     fn pop(&mut self ) ->T ;
}

impl <T> Common<T> for Stack<T> {
    fn pop(&mut self ,) ->T  {
    }

    fn length(&self)->i32 {
        return self.node.len() as i32;
    }
}