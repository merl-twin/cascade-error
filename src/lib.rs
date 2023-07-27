use opt_vec::vec::OptVec;

#[derive(Debug)]
pub struct CascadeError<E: Cascadable> {
    error: E,
    code_trace: OptVec<CodeTrace>,
}
impl<E: Cascadable> CascadeError<E> {
    pub fn inner(&self) -> &E {
        &self.inner
    }
    pub fn into_inner(self) -> E {
        self.error
    }
    pub fn map<F,Q: Cascadable>(self, func: F, trace: CodeTrace) -> CascadeError<Q>
    where F: FnOnce(E) -> Q
    {
        let mut code_trace = self.code_trace;
        code_trace.push(trace);
        CascadeError {
            error: func(self.error),
            code_trace,
        }
    }
    pub fn push(&mut self, trace: CodeTrace) {
        self.code_trace.push(trace);
    }
}

pub trait Cascadable: Sized {
    fn into_cascade(self, trace: CodeTrace) -> CascadeError<Self> {
         CascadeError{ error: self, code_trace: OptVec::from(trace) }
    }
}

#[macro_export]
macro_rules! cascade_new {
    ( ) => {
        |e| { e.into_cascade(code_trace!()) }
    };
    ( $err:expr ) => {
        {
            let tmp = $err;
            tmp.into_cascade(code_trace!())
        }
    };
}

#[macro_export]
macro_rules! cascade_trace {
    ( ) => {
        |mut e: CascadeError<_>| {
            e.push(code_trace!());
            e
        }
    };
}

#[macro_export]
macro_rules! cascade {
    ( $map:expr ) => {
        |e: CascadeError<_>| {
            e.map($map,code_trace!())
        }
    };
}


#[macro_export]
macro_rules! code_trace {
    () => {
        CodeTrace::new(file!(),line!()) 
    };
}

#[derive(Clone,Copy,Eq,Ord,PartialEq,PartialOrd,Hash)]
pub struct CodeTrace {
    file: &'static str,
    line: u32,
}
impl CodeTrace {
    pub fn new(file: &'static str, line: u32) -> CodeTrace {
        CodeTrace { file, line }
    }
}
impl std::fmt::Debug for CodeTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.file, self.line)
    }
}
impl std::fmt::Display for CodeTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.file, self.line)
    }
}

