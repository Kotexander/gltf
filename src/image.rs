#[allow(unused)]
use crate::{buffer, Document, Error, Result};

#[cfg(feature = "import")]
#[cfg_attr(docsrs, doc(cfg(feature = "import")))]
pub use image_crate::DynamicImage;
#[cfg(feature = "extensions")]
use serde_json::{Map, Value};

/// Describes an image data source.
#[derive(Clone, Debug)]
pub enum Source<'a> {
    /// Image data is contained in a buffer view.
    View {
        /// The buffer view containing the encoded image data.
        view: buffer::View<'a>,

        /// The image data MIME type.
        mime_type: &'a str,
    },

    /// Image data is contained in an external data source.
    Uri {
        /// The URI of the external data source.
        uri: &'a str,

        /// The image data MIME type, if provided.
        mime_type: Option<&'a str>,
    },
}

/// Image data used to create a texture.
#[derive(Clone, Debug)]
pub struct Image<'a> {
    /// The parent `Document` struct.
    document: &'a Document,

    /// The corresponding JSON index.
    index: usize,

    /// The corresponding JSON struct.
    json: &'a json::image::Image,
}

impl<'a> Image<'a> {
    /// Constructs an `Image` from owned data.
    pub(crate) fn new(document: &'a Document, index: usize, json: &'a json::image::Image) -> Self {
        Self {
            document,
            index,
            json,
        }
    }

    /// Returns the internal JSON index.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Optional user-defined name for this object.
    #[cfg(feature = "names")]
    #[cfg_attr(docsrs, doc(cfg(feature = "names")))]
    pub fn name(&self) -> Option<&'a str> {
        self.json.name.as_deref()
    }

    /// Returns the image data source.
    pub fn source(&self) -> Source<'a> {
        if let Some(index) = self.json.buffer_view.as_ref() {
            let view = self.document.views().nth(index.value()).unwrap();
            let mime_type = self.json.mime_type.as_ref().map(|x| x.0.as_str()).unwrap();
            Source::View { view, mime_type }
        } else {
            let uri = self.json.uri.as_ref().unwrap();
            let mime_type = self.json.mime_type.as_ref().map(|x| x.0.as_str());
            Source::Uri { uri, mime_type }
        }
    }

    /// Returns extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extensions(&self) -> Option<&Map<String, Value>> {
        let ext = self.json.extensions.as_ref()?;
        Some(&ext.others)
    }

    /// Queries extension data unknown to this crate version.
    #[cfg(feature = "extensions")]
    #[cfg_attr(docsrs, doc(cfg(feature = "extensions")))]
    pub fn extension_value(&self, ext_name: &str) -> Option<&Value> {
        let ext = self.json.extensions.as_ref()?;
        ext.others.get(ext_name)
    }

    /// Optional application specific data.
    pub fn extras(&self) -> &'a json::Extras {
        &self.json.extras
    }
}
