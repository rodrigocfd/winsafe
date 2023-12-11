use crate::co;
use crate::decl::*;
use crate::prelude::*;

pub struct Lnk {
    me: IShellLink,
    pf: IPersistFile,
}

pub trait ShellLinkReadOnly {
    fn get_arguments(&self) -> HrResult<String>;
    fn get_description(&self) -> HrResult<String>;
    fn get_icon_location(&self) -> HrResult<(String, i32)>;
    fn get_path(&self) -> HrResult<String>;
    fn get_show_cmd(&self) -> HrResult<co::SW>;
    fn get_working_directory(&self) -> HrResult<String>;
}

pub trait ShellLinkWriteOnly {
    fn set_arguments(&mut self, path: &str) -> HrResult<()>;
    fn set_description(&mut self, path: &str) -> HrResult<()>;
    fn set_icon_location(&mut self, path: &str, index: i32) -> HrResult<()>;
    fn set_path(&mut self, path: &str) -> HrResult<()>;
    fn set_show_cmd(&mut self, show_cmd: co::SW) -> HrResult<()>;
    fn set_working_directory(&mut self, path: &str) -> HrResult<()>;
    fn save(&self) -> HrResult<()>;
    fn save_as(&self, path: &str) -> HrResult<()>;
}

impl ShellLinkReadOnly for Lnk {
    fn get_arguments(&self) -> HrResult<String> {
        Ok(self.me.GetArguments()?)
    }

    fn get_description(&self) -> HrResult<String> {
        Ok(self.me.GetDescription()?)
    }

    fn get_icon_location(&self) -> HrResult<(String, i32)> {
        Ok(self.me.GetIconLocation()?)
    }

    fn get_path(&self) -> HrResult<String> {
        Ok(self.me.GetPath(None, co::SLGP::UNCPRIORITY)?)
    }

    fn get_show_cmd(&self) -> HrResult<co::SW> {
        Ok(self.me.GetShowCmd()?)
    }

    fn get_working_directory(&self) -> HrResult<String> {
        Ok(self.me.GetWorkingDirectory()?)
    }
}

impl ShellLinkWriteOnly for Lnk {
    fn set_arguments(&mut self, path: &str) -> HrResult<()> {
        Ok(self.me.SetArguments(path)?)
    }

    fn set_description(&mut self, path: &str) -> HrResult<()> {
        Ok(self.me.SetDescription(path)?)
    }

    fn set_icon_location(&mut self, path: &str, index: i32) -> HrResult<()> {
        Ok(self.me.SetIconLocation(path, index)?)
    }

    fn set_path(&mut self, path: &str) -> HrResult<()> {
        Ok(self.me.SetPath(path)?)
    }

    fn set_show_cmd(&mut self, show_cmd: co::SW) -> HrResult<()> {
        Ok(self.me.SetShowCmd(show_cmd)?)
    }

    fn set_working_directory(&mut self, path: &str) -> HrResult<()> {
        Ok(self.me.SetWorkingDirectory(path)?)
    }

    fn save(&self) -> HrResult<()> {
        Ok(self.pf.Save(None, true)?)
    }

    fn save_as(&self, path: &str) -> HrResult<()> {
        Ok(self.pf.Save(Some(path), true)?)
    }
}

pub trait ShellLinkReadWrite: ShellLinkReadOnly + ShellLinkWriteOnly {}
impl ShellLinkReadWrite for Lnk {}

impl Lnk {
    pub fn open_read(file_path: &str) -> HrResult<Box<dyn ShellLinkReadOnly>> {
        let me = CoCreateInstance::<IShellLink>(&co::CLSID::ShellLink, None, co::CLSCTX::INPROC_SERVER)?;
        let pf = me.QueryInterface::<IPersistFile>()?;
        pf.Load(file_path, co::STGM::READ)?;
        let flags = (co::SLR::NO_UI | co::SLR::ANY_MATCH).raw() | (1 << 16);
        let flags_with_timeout = unsafe { co::SLR::from_raw(flags) };
        me.Resolve(&HWND::NULL, flags_with_timeout)?;
        Ok(Box::new(Lnk { me, pf }))
    }

    pub fn open_write(file_path: &str) -> HrResult<Box<dyn ShellLinkReadWrite>> {
        let me = CoCreateInstance::<IShellLink>(&co::CLSID::ShellLink, None, co::CLSCTX::INPROC_SERVER)?;
        let pf = me.QueryInterface::<IPersistFile>()?;
        pf.Load(file_path, co::STGM::READWRITE)?;
        let flags = (co::SLR::NO_UI | co::SLR::ANY_MATCH).raw() | (1 << 16);
        let flags_with_timeout = unsafe { co::SLR::from_raw(flags) };
        me.Resolve(&HWND::NULL, flags_with_timeout)?;
        Ok(Box::new(Lnk { me, pf }))
    }

    pub fn create_new() -> HrResult<Box<dyn ShellLinkReadWrite>> {
        let me = CoCreateInstance::<IShellLink>(&co::CLSID::ShellLink, None, co::CLSCTX::INPROC_SERVER)?;
        let pf = me.QueryInterface::<IPersistFile>()?;
        Ok(Box::new(Lnk { me, pf }))
    }
}