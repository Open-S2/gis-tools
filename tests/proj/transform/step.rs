#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use gistools::proj::{BaseProjection, Id, Method, Proj, Step};

    #[test]
    fn test_step_from_id() {
        // Default case
        let proj = Rc::new(RefCell::new(Proj::default()));
        let method = Method { id: Some(Id::default()), ..Default::default() };
        let step = Step::from_method(&method, proj).unwrap();
        assert_eq!(step, Step::Base(BaseProjection::default().into()));

        // Default case but id fails
        let proj = Rc::new(RefCell::new(Proj::default()));
        let method = Method {
            id: Some(Id { code: "1000000".into(), ..Default::default() }),
            ids: vec![
                Id { code: "1000003".into(), ..Default::default() },
                Id { code: "1000002".into(), ..Default::default() },
                Id { code: "1000001".into(), ..Default::default() },
                Id::default(),
            ],
            ..Default::default()
        };
        let step = Step::from_method(&method, proj).unwrap();
        assert_eq!(step, Step::Base(BaseProjection::default().into()));
    }
}
