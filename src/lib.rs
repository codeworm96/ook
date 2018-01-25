#[macro_export]
macro_rules! Ook {
    // Entry point
    (($($input:tt)*); $($Ooks:tt)*) => {
        Ook!(@e ((), (), ()); ($($input)*); ($($Ooks)*));
    };
    // End
    (@e $status:tt; $input:tt; ()) => {};
    // Move the pointer to the left
    (@e ((), $p:tt, ($($r:tt)*)); $input:tt; (Ook? Ook. $($tail:tt)*)) => {
        Ook!(@e ((), (), ($p $($r)*)); $input; ($($tail)*)); // auto extending
    };
    (@e (($np:tt $($l:tt)*), $p:tt, ($($r:tt)*)); $input:tt; (Ook? Ook. $($tail:tt)*)) => {
        Ook!(@e (($($l)*), $np, ($p $($r)*)); $input; ($($tail)*));
    };
    // Move the pointer to the right
    (@e (($($l:tt)*), $p:tt, ()); $input:tt; (Ook. Ook? $($tail:tt)*)) => {
        Ook!(@e (($p $($l)*), (), ()); $input; ($($tail)*)); // auto extending
    };
    (@e (($($l:tt)*), $p:tt, ($np:tt $($r:tt)*)); $input:tt; (Ook. Ook? $($tail:tt)*)) => {
        Ook!(@e (($p $($l)*), $np, ($($r)*)); $input; ($($tail)*));
    };
    // Increase pointee
    (@e (($($l:tt)*), ($($p:tt)*), ($($r:tt)*)); $input:tt; (Ook. Ook. $($tail:tt)*)) => {
        Ook!(@e (($($l)*), (@ $($p)*), ($($r)*)); $input; ($($tail)*));
    };
    // Decrease pointee
    (@e (($($l:tt)*), (@ $($p:tt)*), ($($r:tt)*)); $input:tt; (Ook! Ook! $($tail:tt)*)) => {
        Ook!(@e (($($l)*), ($($p)*), ($($r)*)); $input; ($($tail)*));
    };
    // Output
    (@e (($($l:tt)*), $p:tt, ($($r:tt)*)); $input:tt; (Ook! Ook. $($tail:tt)*)) => {
        log_syntax!($p);
        Ook!(@e (($($l)*), $p, ($($r)*)); $input; ($($tail)*));
    };
    // Input
    (@e (($($l:tt)*), $p:tt, ($($r:tt)*)); ($cur:tt $($input:tt)*); (Ook. Ook! $($tail:tt)*)) => {
        Ook!(@e (($($l)*), $cur, ($($r)*)); ($($input)*); ($($tail)*));
    };
    // loop begin
    (@e (($($l:tt)*), (), ($($r:tt)*)); $input:tt; (Ook! Ook? $($tail:tt)*)) => {
        Ook!(@s (($($l)*), (), ($($r)*)); $input; (); ($($tail)*)); // skip the loop
    };
    (@e (($($l:tt)*), ($($p:tt)+), ($($r:tt)*)); $input:tt; (Ook! Ook? $($tail:tt)*)) => {
        Ook!(@x (($($l)*), ($($p)+), ($($r)*)); $input; (); (); ($($tail)*)); // do the loop
    };
    // do the loop
    (@x $status:tt; $input:tt; (); ($($buf:tt)*); (Ook? Ook! $($tail:tt)*)) => {
        Ook!(@e $status; $input; ($($buf)* Ook! Ook? $($buf)* Ook? Ook! $($tail)*));
    };
    (@x $status:tt; $input:tt; ($($depth:tt)*); ($($buf:tt)*); (Ook! Ook? $($tail:tt)*)) => {
        Ook!(@x $status; $input; (@ $($depth)*); ($($buf)* Ook! Ook?); ($($tail)*));
    };
    (@x $status:tt; $input:tt; (@ $($depth:tt)*); ($($buf:tt)*); (Ook? Ook! $($tail:tt)*)) => {
        Ook!(@x $status; $input; ($($depth)*); ($($buf)* Ook? Ook!); ($($tail)*));
    };
    (@x $status:tt; $input:tt; $depth:tt; ($($buf:tt)*); (Ook $op0:tt Ook $op1:tt $($tail:tt)*)) => {
        Ook!(@x $status; $input; $depth; ($($buf)* Ook $op0 Ook $op1); ($($tail)*));
    };
    // skip the loop
    (@s $status:tt; $input:tt; (); (Ook? Ook! $($tail:tt)*)) => {
        Ook!(@e $status; $input; ($($tail)*));
    };
    (@s $status:tt; $input:tt; ($($depth:tt)*); (Ook! Ook? $($tail:tt)*)) => {
        Ook!(@s $status; $input; (@ $($depth)*); ($($tail)*));
    };
    (@s $status:tt; $input:tt; (@ $($depth:tt)*); (Ook? Ook! $($tail:tt)*)) => {
        Ook!(@s $status; $input; ($($depth)*); ($($tail)*));
    };
    (@s $status:tt; $input:tt; $depth:tt; (Ook $op0:tt Ook $op1:tt $($tail:tt)*)) => {
        Ook!(@s $status; $input; $depth; ($($tail)*));
    };
}