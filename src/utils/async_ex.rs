pub trait AsyncResult<T, Ok, Err>
where
    T: Future<Output = Result<Ok, Err>>,
{
    fn await_map_err<NewErr, F: FnOnce(Err) -> NewErr>(
        self,
        op: F,
    ) -> impl Future<Output = Result<Ok, NewErr>>;

    fn await_inspect_err<F: FnOnce(&Err)>(self, op: F) -> impl Future<Output = Result<Ok, Err>>;
}

impl<T, Ok, Err> AsyncResult<T, Ok, Err> for T
where
    T: Future<Output = Result<Ok, Err>>,
{
    async fn await_map_err<NewErr, F: FnOnce(Err) -> NewErr>(self, op: F) -> Result<Ok, NewErr> {
        self.await.map_err(op)
    }

    async fn await_inspect_err<F: FnOnce(&Err)>(self, op: F) -> Result<Ok, Err> {
        self.await.inspect_err(op)
    }
}
