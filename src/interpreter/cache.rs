// A macro that caches a method call for a struct
//
// Requires a backing store
//
//     $cache: <Cell<Option<T>>
//
// where T implements Copy
//
// Essentially equivalent to `@cache ||= expr` in Ruby

macro_rules! cache {
    ($self:ident.$cache:ident, $e:expr) => {
        match $self.$cache.get() {
            Some(cached) => cached,
            None => {
                let val = $e;
                $self.$cache.set(Some(val));
                val
            }
        }
    };
}
