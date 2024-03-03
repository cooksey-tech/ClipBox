use std::path::PathBuf;

use windows_sys::{core::HRESULT, Win32::System::Com::{IAdviseSink, IEnumFORMATETC, IEnumSTATDATA, FORMATETC, STGMEDIUM}};


/// Built based on IDataObject interface from the Windows API
/// This is to operate as close to a COM object as possible
trait IDataObject {
    fn GetData(pformatetc: FORMATETC, pmedium: STGMEDIUM) -> HRESULT;
    fn GetDataHere(pformatetc: FORMATETC, pmedium: STGMEDIUM) -> HRESULT;
    fn QueryGetData(pformatetc: FORMATETC) -> HRESULT;
    fn GetCanonicalFormatEtc(pformatetcIn: FORMATETC, pformatetcOut: FORMATETC) -> HRESULT;
    /// Sets and stores data in a specific format
    fn SetData(pformatetc: FORMATETC, pmedium: STGMEDIUM, fRelease: bool) -> HRESULT;
    fn EnumFormatEtc(dwDirection: u32, ppenumFormatEtc: *mut IEnumFORMATETC) -> HRESULT;
    fn DAdvise(pformatetc: FORMATETC, advf: u32, pAdvSink: *mut IAdviseSink, pdwConnection: *mut u32) -> HRESULT;
    fn DUnadvise(dwConnection: u32) -> HRESULT;
    fn EnumDAdvise(ppenumAdvise: *mut *mut IEnumSTATDATA) -> HRESULT;
}

#[repr(C)]
struct DataObject{
    file_path: PathBuf
}

impl IDataObject for DataObject {
    fn GetData(pformatetc: FORMATETC, pmedium: STGMEDIUM) -> HRESULT {
        unimplemented!()
    }
    fn GetDataHere(pformatetc: FORMATETC, pmedium: STGMEDIUM) -> HRESULT {
        unimplemented!()
    }
    fn QueryGetData(pformatetc: FORMATETC) -> HRESULT {
        unimplemented!()
    }
    fn GetCanonicalFormatEtc(pformatetcIn: FORMATETC, pformatetcOut: FORMATETC) -> HRESULT {
        unimplemented!()
    }
    fn SetData(pformatetc: FORMATETC, pmedium: STGMEDIUM, fRelease: bool) -> HRESULT {
        unimplemented!()
    }
    fn EnumFormatEtc(dwDirection: u32, ppenumFormatEtc: *mut IEnumFORMATETC) -> HRESULT {
        unimplemented!()
    }
    fn DAdvise(pformatetc: FORMATETC, advf: u32, pAdvSink: *mut IAdviseSink, pdwConnection: *mut u32) -> HRESULT {
        unimplemented!()
    }
    fn DUnadvise(dwConnection: u32) -> HRESULT {
        unimplemented!()
    }
    fn EnumDAdvise(ppenumAdvise: *mut *mut IEnumSTATDATA) -> HRESULT {
        unimplemented!()
    }
}
