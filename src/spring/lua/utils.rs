use nvim_oxi::mlua::{lua, FromLua};

use crate::spring::errors::SpringtimeError;

pub struct LuaUtils;

pub struct Module<'a>(pub &'a str);
pub struct Variable<'a>(pub &'a str);

impl LuaUtils {
    pub fn get_springtime_plugin_path() -> Result<String, SpringtimeError> {
        let lua_path = Self::get_lua_module(
            Module("require'springtime.util'.lua_springtime_path"),
            Variable("path"),
        )?;
        Ok(lua_path)
    }

    pub fn get_springtime_log_file() -> Result<String, SpringtimeError> {
        let lua_log_file = Self::get_lua_module(
            Module("require'springtime.util'.springtime_log_file"),
            Variable("log_file"),
        )?;
        Ok(lua_log_file)
    }

    pub fn get_lua_module<'lua, V: FromLua<'lua>>(
        module: Module,
        variable: Variable,
    ) -> Result<V, SpringtimeError> {
        let lua = lua();
        lua.load(format!("{} = {}", variable.0, module.0))
            .exec()
            .map_err(|_| SpringtimeError::Generic(format!("Lua {} does not exist", module.0)))?;

        let lua_table: V = lua.globals().get(variable.0).unwrap();
        Ok(lua_table)
    }
}
