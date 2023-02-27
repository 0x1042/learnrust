# example 


## for 语法

|语法|等价于|访问级别|
|----|----|----|
|`for item in collection`|`for item in IntoIterator::into_iter(collection)`|拥有所有权,也就是`for`语句结束之后，`collection`已经无效|
|`for item in &collection`|`for item in collection.iter()`|只读，语句结束，`collection`有效|
|`for item in &mut collection`|`for item in collection.iter_mut()`|读写，语句结束，`collection`有效|