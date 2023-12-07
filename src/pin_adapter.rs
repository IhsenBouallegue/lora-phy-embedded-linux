pub struct WithWait<T> {
    wrapped: T,
}

impl<T> WithWait<T> {
    pub fn new(wrapped: T) -> Self {
        Self { wrapped }
    }
}

impl<T> embedded_hal::digital::ErrorType for WithWait<T>
where
    T: embedded_hal::digital::ErrorType,
{
    type Error = T::Error;
}

impl<T> embedded_hal_async::digital::Wait for WithWait<T>
where
    T: embedded_hal::digital::InputPin + 'static,
{
    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}
