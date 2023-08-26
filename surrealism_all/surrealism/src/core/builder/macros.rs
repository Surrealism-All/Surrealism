//! # macros for wrapper
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/8/26
//! @version:0.0.1
//! @description:
//! ```

#[macro_export]
macro_rules! return_impl {
    ($Wrapper:tt) => {
        impl ReturnImpl for $Wrapper {
            fn return_type(&mut self, return_type: ReturnType) -> &mut Self {
                let _ = self.return_type.replace(return_type);
                self
            }
        }
    };
}

#[macro_export]
macro_rules! timeout_impl {
    ($Wrapper:tt) => {
        impl TimeoutImpl for $Wrapper {
            fn timeout_from(&mut self, timeout: TimeOut) -> &mut Self {
                let _ = self.timeout.replace(timeout);
                self
            }
            fn timeout(&mut self, timeout: usize, unit: TimeUnit) -> &mut Self {
                self.timeout_from(TimeOut::new(timeout, unit))
            }
        }
    };
}

#[macro_export]
macro_rules! parallel_impl {
    ($Wrapper:tt) => {
        impl ParallelImpl for $Wrapper {
            fn parallel(&mut self) -> &mut Self {
                self.parallel = true;
                self
            }
        }
    };
}

#[macro_export]
macro_rules! table_impl {
    ($Wrapper:tt) => {
        impl TableImpl for $Wrapper {
            fn table(&mut self, table: &str) -> &mut Self {
                self.table.table(table);
                self
            }

            fn id(&mut self, id: SurrealID) -> &mut Self {
                self.table.id(id);
                self
            }
        }
    };
}

#[macro_export]
macro_rules! return_lifetime_impl {
    ($Wrapper:tt) => {
        impl<'w> ReturnImpl for $Wrapper<'w> {
            fn return_type(&mut self, return_type: ReturnType) -> &mut Self {
                let _ = self.return_type.replace(return_type);
                self
            }
        }
    };
}

#[macro_export]
macro_rules! timeout_lifetime_impl {
    ($Wrapper:tt) => {
        impl<'w> TimeoutImpl for $Wrapper<'w> {
            fn timeout_from(&mut self, timeout: TimeOut) -> &mut Self {
                let _ = self.timeout.replace(timeout);
                self
            }
            fn timeout(&mut self, timeout: usize, unit: TimeUnit) -> &mut Self {
                self.timeout_from(TimeOut::new(timeout, unit))
            }
        }
    };
}

#[macro_export]
macro_rules! parallel_lifetime_impl {
    ($Wrapper:tt) => {
        impl<'w> ParallelImpl for $Wrapper<'w> {
            fn parallel(&mut self) -> &mut Self {
                self.parallel = true;
                self
            }
        }
    };
}

#[macro_export]
macro_rules! table_lifetime_impl {
    ($Wrapper:tt) => {
        impl<'w> TableImpl for $Wrapper<'w> {
            fn table(&mut self, table: &str) -> &mut Self {
                self.table.table(table);
                self
            }

            fn id(&mut self, id: SurrealID) -> &mut Self {
                self.table.id(id);
                self
            }
        }
    };
}


