use serde_json::{value::Index, Value};

use super::EventBuildError;

#[derive(Debug, Clone)]
pub struct NoticeEvent {
    /// 事件发生的时间戳
    pub time: i64,
    /// 收到事件的机器人 登陆号
    pub self_id: i64,
    /// 上报类型
    pub post_type: String,
    /// 通知类型
    pub notice_type: String,

    /// 原始的onebot消息，已处理成json格式
    pub original_json: Value,
}
impl NoticeEvent {
    pub(crate) fn new(msg: &str) -> Result<NoticeEvent, EventBuildError> {
        let temp: Value =
            serde_json::from_str(msg).map_err(|e| EventBuildError::ParseError(e.to_string()))?;
        let time = temp
            .get("time")
            .and_then(Value::as_i64)
            .ok_or(EventBuildError::ParseError("time".to_string()))?;
        let self_id = temp
            .get("self_id")
            .and_then(Value::as_i64)
            .ok_or(EventBuildError::ParseError("self_id".to_string()))?;
        let post_type = temp
            .get("post_type")
            .and_then(Value::as_str)
            .map(String::from)
            .ok_or(EventBuildError::ParseError("post_type".to_string()))?;
        let notice_type = temp
            .get("notice_type")
            .and_then(Value::as_str)
            .map(String::from)
            .ok_or(EventBuildError::ParseError("notice_type".to_string()))?;
        Ok(NoticeEvent {
            time,
            self_id,
            post_type,
            notice_type,
            original_json: temp,
        })
    }
}

impl NoticeEvent {
    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        self.original_json.get(index)
    }

    pub fn get_mut<I: Index>(&mut self, index: I) -> Option<&mut Value> {
        self.original_json.get_mut(index)
    }
}

impl<I> std::ops::IndexMut<I> for NoticeEvent
where
    I: Index,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.original_json[index]
    }
}

impl<I> std::ops::Index<I> for NoticeEvent
where
    I: Index,
{
    type Output = Value;

    fn index(&self, index: I) -> &Self::Output {
        &self.original_json[index]
    }
}
