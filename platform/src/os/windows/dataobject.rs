#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use {
    std::cell::RefCell,
    crate::{
        windows::{
            core,
            core::implement,
            Win32::{
                System::{
                    Ole::{
                        CF_HDROP,
                    },
                    Com::{
                        IDataObject_Impl,
                        IEnumFORMATETC,
                        IDataObject,
                        FORMATETC,
                        STGMEDIUM,
                        STGMEDIUM_0,
                        IAdviseSink,
                        IEnumSTATDATA,
                        DATADIR_GET,
                        TYMED_HGLOBAL,
                        DVASPECT_CONTENT,
                    },
                },
                Foundation::{
                    BOOL,
                    S_OK,
                    E_NOTIMPL,
                    OLE_E_ADVISENOTSUPPORTED,
                    DATA_S_SAMEFORMATETC,
                    DV_E_FORMATETC,
                    DV_E_DVASPECT,
                    DV_E_LINDEX,
                    DV_E_TYMED,
                    E_UNEXPECTED,
                },
            },
        },
        os::windows::{
            enumformatetc::*,
            dropfiles::*,
        },
        event::DragItem,
    },
};
/*
// This is a reimplementation of windows-rs IDataObject that refers to the reimplemented IEnumFORMATETC from enumformatetc.rs

#[repr(transparent)]pub struct IDataObject(core::IUnknown);
impl IDataObject {
    pub unsafe fn GetData(&self, pformatetcin: *const FORMATETC) -> core::Result<STGMEDIUM> {
        let mut result__ = ::std::mem::zeroed();
        (core::Interface::vtable(self).GetData)(core::Interface::as_raw(self), pformatetcin, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> core::Result<()> {
        (core::Interface::vtable(self).GetDataHere)(core::Interface::as_raw(self), pformatetc, pmedium).ok()
    }
    pub unsafe fn QueryGetData(&self, pformatetc: *const FORMATETC) -> core::HRESULT {
        (core::Interface::vtable(self).QueryGetData)(core::Interface::as_raw(self), pformatetc)
    }
    pub unsafe fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> core::HRESULT {
        (core::Interface::vtable(self).GetCanonicalFormatEtc)(core::Interface::as_raw(self), pformatectin, pformatetcout)
    }
    pub unsafe fn SetData<P0>(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: P0) -> core::Result<()>
    where
        P0: core::IntoParam<BOOL>,
    {
        (core::Interface::vtable(self).SetData)(core::Interface::as_raw(self), pformatetc, pmedium, frelease.into_param().abi()).ok()
    }
    pub unsafe fn EnumFormatEtc(&self, dwdirection: u32) -> core::Result<IEnumFORMATETC> {
        let mut result__ = ::std::mem::zeroed();
        (core::Interface::vtable(self).EnumFormatEtc)(core::Interface::as_raw(self), dwdirection, &mut result__).from_abi(result__)
    }
    pub unsafe fn DAdvise<P0>(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: P0) -> core::Result<u32>
    where
        P0: core::IntoParam<IAdviseSink>,
    {
        let mut result__ = ::std::mem::zeroed();
        (core::Interface::vtable(self).DAdvise)(core::Interface::as_raw(self), pformatetc, advf, padvsink.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DUnadvise(&self, dwconnection: u32) -> core::Result<()> {
        (core::Interface::vtable(self).DUnadvise)(core::Interface::as_raw(self), dwconnection).ok()
    }
    pub unsafe fn EnumDAdvise(&self) -> core::Result<IEnumSTATDATA> {
        let mut result__ = ::std::mem::zeroed();
        (core::Interface::vtable(self).EnumDAdvise)(core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
impl ::core::cmp::Eq for IDataObject {}
impl ::core::cmp::PartialEq for IDataObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::clone::Clone for IDataObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::fmt::Debug for IDataObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataObject").field(&self.0).finish()
    }
}
unsafe impl core::Interface for IDataObject {
    type Vtable = IDataObject_Vtbl;
}
unsafe impl core::ComInterface for IDataObject {
    const IID: core::GUID = core::GUID::from_u128(0x0000010e_0000_0000_c000_000000000046);
}

impl core::CanInto<core::IUnknown> for IDataObject { }


#[repr(C)]
pub struct IDataObject_Vtbl {
    pub base__: core::IUnknown_Vtbl,
    pub GetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetcin: *const FORMATETC, pmedium: *mut STGMEDIUM) -> core::HRESULT,
    pub GetDataHere: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> core::HRESULT,
    pub QueryGetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC) -> core::HRESULT,
    pub GetCanonicalFormatEtc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> core::HRESULT,
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: BOOL) -> core::HRESULT,
    pub EnumFormatEtc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwdirection: u32, ppenumformatetc: *mut *mut ::core::ffi::c_void) -> core::HRESULT,
    pub DAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, advf: u32, padvsink: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> core::HRESULT,
    pub DUnadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwconnection: u32) -> core::HRESULT,
    pub EnumDAdvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumadvise: *mut *mut ::core::ffi::c_void) -> core::HRESULT,
}

pub trait IDataObject_Impl: Sized {
    fn GetData(&self, pformatetcin: *const FORMATETC) -> core::Result<STGMEDIUM>;
    fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> core::Result<()>;
    fn QueryGetData(&self, pformatetc: *const FORMATETC) -> core::HRESULT;
    fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> core::HRESULT;
    fn SetData(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: BOOL) -> core::Result<()>;
    fn EnumFormatEtc(&self, dwdirection: u32) -> core::Result<IEnumFORMATETC>;
    fn DAdvise(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: ::core::option::Option<&IAdviseSink>) -> core::Result<u32>;
    fn DUnadvise(&self, dwconnection: u32) -> core::Result<()>;
    fn EnumDAdvise(&self) -> core::Result<IEnumSTATDATA>;
}

impl core::RuntimeName for IDataObject {}

impl IDataObject_Vtbl {
    pub const fn new<Identity: core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>() -> IDataObject_Vtbl {
        unsafe extern "system" fn GetData<Identity: core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetcin: *const FORMATETC, pmedium: *mut STGMEDIUM) -> core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetData(::core::mem::transmute_copy(&pformatetcin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmedium, ::core::mem::transmute(ok__));
                    core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataHere<Identity: core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDataHere(::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pmedium)).into()
        }
        unsafe extern "system" fn QueryGetData<Identity: core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC) -> core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryGetData(::core::mem::transmute_copy(&pformatetc))
        }
        unsafe extern "system" fn GetCanonicalFormatEtc<Identity: core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCanonicalFormatEtc(::core::mem::transmute_copy(&pformatectin), ::core::mem::transmute_copy(&pformatetcout))
        }
        unsafe extern "system" fn SetData<Identity: core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: BOOL) -> core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pmedium), ::core::mem::transmute_copy(&frelease)).into()
        }
        unsafe extern "system" fn EnumFormatEtc<Identity: core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdirection: u32, ppenumformatetc: *mut *mut ::core::ffi::c_void) -> core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumFormatEtc(::core::mem::transmute_copy(&dwdirection)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumformatetc, ::core::mem::transmute(ok__));
                    core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DAdvise<Identity: core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, advf: u32, padvsink: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DAdvise(::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&advf), core::from_raw_borrowed(&padvsink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconnection, ::core::mem::transmute(ok__));
                    core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DUnadvise<Identity: core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DUnadvise(::core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumDAdvise<Identity: core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut *mut ::core::ffi::c_void) -> core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumDAdvise() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumadvise, ::core::mem::transmute(ok__));
                    core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetData: GetData::<Identity, Impl, OFFSET>,
            GetDataHere: GetDataHere::<Identity, Impl, OFFSET>,
            QueryGetData: QueryGetData::<Identity, Impl, OFFSET>,
            GetCanonicalFormatEtc: GetCanonicalFormatEtc::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            EnumFormatEtc: EnumFormatEtc::<Identity, Impl, OFFSET>,
            DAdvise: DAdvise::<Identity, Impl, OFFSET>,
            DUnadvise: DUnadvise::<Identity, Impl, OFFSET>,
            EnumDAdvise: EnumDAdvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &core::GUID) -> bool {
        iid == &<IDataObject as core::ComInterface>::IID
    }
}*/
/*
implement_com!{
    for_struct: DragItem,
    identity: IDataObject,
    wrapper_struct: DragItem_Com,
    interface_count: 1,
    interfaces: {
        0: IDataObject
    }
}
*/
#[implement(IDataObject)]
pub struct DragItemWindows(pub DragItem);
// IDataObject implementation for DragItem

#[allow(non_snake_case)]
impl IDataObject_Impl for DragItemWindows {

    fn GetData(&self, pformatetc: *const FORMATETC) -> core::Result<STGMEDIUM> {

        // if no format was supplied, return DV_E_FORMATETC
        if pformatetc == std::ptr::null_mut() {
            Err(DV_E_FORMATETC.into())
        }

        else {

            // if the format is not a CF_HDROP, deny
            if unsafe { (*pformatetc).cfFormat } != CF_HDROP.0 as u16 {
                Err(DV_E_FORMATETC.into())
            }

            // if the format's aspect is not DVASPECT_CONTENT, deny
            else if unsafe { (*pformatetc).dwAspect } != DVASPECT_CONTENT.0 as u32 {
                Err(DV_E_DVASPECT.into())
            }

            // if the index is not -1, deny
            else if unsafe { (*pformatetc).lindex } != -1 {
                Err(DV_E_LINDEX.into())
            }

            // if the medium is not a HGLOBAL, deny
            else if unsafe { (*pformatetc).tymed } != TYMED_HGLOBAL.0 as u32 {
                Err(DV_E_TYMED.into())
            }

            else {
                let hglobal_opt = create_hglobal_for_dragitem(&self.0);

                if let Some(hglobal) = hglobal_opt {
                    Ok(STGMEDIUM {
                        tymed: TYMED_HGLOBAL.0 as u32,
                        u: STGMEDIUM_0 { hGlobal: hglobal, },
                        pUnkForRelease: std::mem::ManuallyDrop::new(None),
                    })
                }
                else {
                    Err(E_UNEXPECTED.into())
                }
            }
        }
    }

    fn GetDataHere(&self, _: *const FORMATETC, _: *mut STGMEDIUM) -> core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn QueryGetData(&self, pformatetc: *const FORMATETC) -> core::HRESULT {

        // if no format was supplied, return DV_E_FORMATETC
        if pformatetc == std::ptr::null_mut() {
            DV_E_FORMATETC
        }

        else {

            // if the format is not a CF_HDROP, deny
            if unsafe { (*pformatetc).cfFormat } != CF_HDROP.0 as u16 {
                DV_E_FORMATETC
            }

            // if the format's aspect is not DVASPECT_CONTENT, deny
            else if unsafe { (*pformatetc).dwAspect } != DVASPECT_CONTENT.0 as u32 {
                DV_E_DVASPECT
            }

            // if the index is not -1, deny
            else if unsafe { (*pformatetc).lindex } != -1 {
                DV_E_LINDEX
            }

            // if the medium is not a HGLOBAL, deny
            else if unsafe { (*pformatetc).tymed } != TYMED_HGLOBAL.0 as u32 {
                DV_E_TYMED
            }

            else {
                S_OK
            }
        }
    }

    fn GetCanonicalFormatEtc(&self, pformatetcin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> core::HRESULT {

        // if no format was supplied, return DV_E_FORMATETC
        if pformatetcin == std::ptr::null_mut() {
            return DV_E_FORMATETC
        }

        // just copy the format and zero the device pointer
        unsafe { 
            *pformatetcout = *pformatetcin;
            (*pformatetcout).ptd = std::ptr::null_mut();
        }

        DATA_S_SAMEFORMATETC
    }

    fn SetData(&self, _: *const FORMATETC, _: *const STGMEDIUM, _: BOOL) -> core::Result<()> {
        Err(E_NOTIMPL.into())
    }

    fn EnumFormatEtc(&self, dwdirection: u32) -> core::Result<IEnumFORMATETC> {
        if dwdirection != DATADIR_GET.0 as u32 {
            Err(E_NOTIMPL.into())
        }
        else {
            let formats = vec![
                FORMATETC {
                    cfFormat: CF_HDROP.0,
                    ptd: std::ptr::null_mut(),
                    dwAspect: DVASPECT_CONTENT.0,
                    lindex: -1,
                    tymed: TYMED_HGLOBAL.0 as u32,
                },
            ];
            let enum_format_etc: IEnumFORMATETC = EnumFormatEtc { formats,index: RefCell::new(0), }.into();
            Ok(enum_format_etc)
        }
    }

    fn DAdvise(&self, _: *const FORMATETC, _: u32, _: ::core::option::Option<&IAdviseSink>) -> core::Result<u32> {
        Err(OLE_E_ADVISENOTSUPPORTED.into())
    }

    fn DUnadvise(&self, _: u32) -> core::Result<()> {
        Err(OLE_E_ADVISENOTSUPPORTED.into())
    }

    fn EnumDAdvise(&self) -> core::Result<IEnumSTATDATA> {
        Err(OLE_E_ADVISENOTSUPPORTED.into())
    }
}
