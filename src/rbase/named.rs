use crate::rbase;
use crate::rbytes::rbuffer::RBuffer;
use crate::rbytes::Unmarshaler;
use crate::root;
use crate::root::traits::Object;

/// The TNamed class is the base class for all named ROOT classes
/// A TNamed contains the essential elements (name, title)
/// to identify a derived object in containers, directories and files.
/// Most member functions defined in this base class are in general
/// overridden by the derived classes.
#[derive(Default, Debug, Clone)]
pub struct Named {
    pub(crate) obj: rbase::Object,
    pub(crate) name: String,
    pub(crate) title: String,
}

impl root::traits::Object for Named {
    fn class(&self) -> &'_ str {
        "TNamed"
    }
}

impl root::traits::Named for Named {
    fn name(&self) -> &'_ str {
        &self.name
    }

    fn title(&self) -> &'_ str {
        &self.title
    }
}

impl Named {
    pub(crate) fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub(crate) fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }
}

impl Unmarshaler for Named {
    fn unmarshal(&mut self, r: &mut RBuffer) -> crate::rbytes::Result<()> {
        let hdr = r.read_header(self.class())?;

        {
            // r.read_object(&mut self.obj)?;

            self.obj = r.read_object_into::<rbase::Object>()?;
        }

        self.name = r.read_string()?.into();
        self.title = r.read_string()?.into();

        r.check_header(&hdr)?;

        Ok(())

        // todo!()
    }
}
