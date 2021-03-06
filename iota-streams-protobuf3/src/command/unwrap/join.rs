use failure::Fallible;

use super::Context;
use crate::{
    command::Join,
    io,
    types::{
        LinkStore,
        SkipFallback,
    },
};
use iota_streams_core::{
    sponge::prp::PRP,
    tbits::word::SpongosTbitWord,
};

impl<'a, TW, F, L: SkipFallback<TW, F>, S: LinkStore<TW, F, L>, IS: io::IStream<TW>> Join<&'a mut L, &S>
    for Context<TW, F, IS>
where
    TW: SpongosTbitWord,
    F: PRP<TW>,
{
    fn join(&mut self, store: &S, link: &'a mut L) -> Fallible<&mut Self> {
        //TODO: Move `skip` out of `join` and `skip` links explicitly.
        // That way it's easier to handle the case when the link is not found
        // and calling function can try to fetch and parse message for the link.
        //TODO: Implement a strategy (depth of recursion or max number of retries)
        // for such cases.
        link.unwrap_skip(self)?;
        //TODO: Return and use info.
        let (mut s, _i) = store.lookup(link)?;
        self.spongos.join(&mut s);
        Ok(self)
    }
}
/*
impl<'a, L, S: LinkStore<L>, IS: io::IStream<TW>> Join<&'a mut L, &S> for Context<TW, F, IS> where
    Self: Skip<&'a mut L>,
{
    fn join(&mut self, store: &S, link: &'a mut L) -> Fallible<&mut Self> {
        self.skip(link)?;
        let (mut s, i) = store.lookup(link)?;
        self.spongos.join(&mut s);
        Ok(self)
    }
}
 */
