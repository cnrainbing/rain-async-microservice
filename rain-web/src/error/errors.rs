use async_graphql::{Error as AgError, ErrorExtensions};
use thiserror::Error;
use validator::ValidationErrors;


/// 定义错误枚举
#[derive(Debug, Error)]
pub enum AppError {
    #[error("服务器内部错误")]
    InternalError,

    #[error("客户端错误")]
    ClientError,

    #[error("用户名已存在")]
    UsernameAlreadyExists,

    #[error("邮箱已存在")]
    EmailAlreadyExists,

    #[error("请求参数错误")]
    RequestParameterError,

    #[error("用户名或密码错误")]
    UsernameOrPasswordError,
}

impl AppError {
    //  2021-04-25 00:16:38 错误处理先这样吧 以后有了更好的再处理 总归服务器的错误不应该暴露到客户端去.
    /// 返回错误扩展并输出日志的闭包
    pub fn log_extend(self) -> Box<dyn FnOnce(anyhow::Error) -> AgError> {
        Box::new(move |error| {
            // 日志打印输出的位置包路径显然不对, 思考能不能找到最初的位置
            log::error!("{:#}", error);
            self.extend()
        })
    }

    /// 返回错误扩展并输出日志的闭包
    pub fn validation_extend(self) -> Box<dyn FnOnce(ValidationErrors) -> AgError> {
        Box::new(move |error| {
            log::warn!("{:?}", error);
            self.extend_with(|_, e| {
                e.set("code", "A0001");
                for (column, error_vec) in error.field_errors() {
                    let result = serde_json::to_string(&error_vec.first()).unwrap_or_default();
                    e.set(column, result);
                }
            })
        })
    }
}

/// 实现错误扩展
impl ErrorExtensions for AppError {
    /// 定义基本扩展
    fn extend(&self) -> AgError {
        self.extend_with(|error, e| {
            match error {
                // 在返回给客户端的新增中新增了 code 业务状态码, 作为业务状态梳理
                AppError::InternalError => e.set("code", "B0001"),
                AppError::ClientError => e.set("code", "A0001"),
                AppError::RequestParameterError => e.set("code", "A0002"),
                AppError::UsernameAlreadyExists => e.set("code", "A0003"),
                AppError::EmailAlreadyExists => e.set("code", "A0004"),
                AppError::UsernameOrPasswordError => e.set("code", "A0005"),
            }
        })
    }

    fn extend_with<C>(self, cb: C) -> AgError
        where
            C: FnOnce(&Self, &mut async_graphql::ErrorExtensionValues),
    {
        let message = self.extend().message;
        let mut extensions = self.extend().extensions.unwrap_or_default();
        cb(&self, &mut extensions);
        AgError {
            message,
            extensions: Some(extensions),
        }
    }
}