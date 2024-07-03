use crate::domain::gateway::Browser;
use crate::domain::Url;

pub(crate) struct BrowserImpl;

impl Browser for BrowserImpl {
    fn browser_single_url(&self, url: &Url) -> anyhow::Result<()> {
        Ok(webbrowser::open(url)?) // Deref coercion
                                   //If T implements Deref<Target = U>, and v is a value of type T, then:
                                   //    In immutable contexts, *v (where T is neither a reference nor a raw pointer) is equivalent to *Deref::deref(&v).
                                   //    Values of type &T are coerced to values of type &U
                                   //    T implicitly implements all the methods of the type U which take the &self receiver.
    }
}
