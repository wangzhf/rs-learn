mod test_debug_and_display;
mod test_primary;
mod test_struct;
mod test_control_flow;
mod test_fn;
mod test_trait;
mod test_generics;
mod test_in_out_stream;
mod test_file_in;
mod test_loop;
mod test_string;
mod test_operator;
mod test_match;
mod test_pattern;
mod test_borrowing;
mod test_lifetime;
mod test_closure;
mod test_vec;
mod test_hashmap;
mod test_iterator;
mod test_macro;
mod test_rc;
mod test_arc;
mod test_mutex;
mod test_cell;
mod test_other_trait;
mod test_asref;
mod test_borrow;
mod test_deref;
mod test_cow;
mod test_thread;
mod test_share_mem;
mod test_notify;
mod test_parallel;
mod test_unsafe;
mod test_any;
mod data_structure;
mod grep;

mod domain;

fn main() {

    // test_debug_and_display::test_display();
    // test_debug_and_display::test_debug_and_display();

    // test_primary::test_primary();
    // test_primary::test_array();
    // test_primary::test_slice();
    // test_primary::test_vec();

    // test_struct::test_struct();
    // test_struct::test_struct2();
    // test_struct::test_struct3();

    // test_control_flow::test_if();
    // test_control_flow::test_for();
    // test_control_flow::test_match();


    // test_fn::test_fn();
    // test_fn::test_closure();
    // test_fn::test_high_order_fn();
    // test_fn::test_method();

    // test_trait::test_trait1();
    // test_trait::test_trait2();

    // test_generics::test_generics1();
    // test_generics::test_add();
    // test_generics::test_add3();

    // test_in_out_stream::read_input();
    // test_in_out_stream::read_input2();
    // test_in_out_stream::out();
    // test_in_out_stream::read_and_out();


    // test_file_in::read_file();
    // test_file_in::out_file();

    // test_loop::test_for_int();
    // test_loop::test_for_index();
    // test_loop::test_line();
    // test_loop::test_while();
    // test_loop::test_while_init();
    // test_loop::test_loop_init();
    // test_loop::test_break();
    // test_loop::test_continue();
    // test_loop::test_label();


    // test_string::test_str();
    // test_string::test_string();

    // test_operator::test_operator_override();
    // test_operator::test_format_macro();


    // test_match::test_match();
    // test_match::test_match2();

    // test_pattern::test_pattern();
    // test_pattern::test_pattern2();
    // test_pattern::test_pattern3();
    // test_pattern::test_pattern4();
    // test_pattern::test_pattern5();
    // test_pattern::test_ref();
    // test_pattern::test_param_bind();
    // test_pattern::test_if();


    // test_borrowing::test_borrowing();
    // test_borrowing::test_borrowing2();
    // test_borrowing::test_borrowing3();
    // test_borrowing::test_borrowing4();

    // test_lifetime::test_lifetime();

    // test_closure::test_closure();
    // test_closure::test_closure2();
    // test_closure::test_closure3();
    // test_closure::test_closure4();
    // test_closure::test_closure5();
    // test_closure::test_closure_as_parameter();
    // test_closure::test_fn_point_closure();
    // test_closure::test_closure_as_return_value();

    // test_vec::test_vec_init();
    // test_vec::test_vec_access();
    // test_vec::test_vec_iterator();
    // test_vec::test_push_efficiency();


    // test_hashmap::test_hashmap();
    // test_hashmap::test_entry();

    // test_iterator::test_for();
    // test_iterator::test_vec();
    // test_iterator::test_collect();
    // test_iterator::test_adapter();
    // test_iterator::test_other();


    // 测试模块
    // let u = domain::user::User::new("jack".to_string(), 20);
    // println!("name: {}", u.get_name());

    // domain::d::f::g::print_g();
    // domain::g::print_g();


    // macro
    // test_macro::test_macro();


    // test_rc::test_rc();
    // test_rc::test_weak();

    // test_arc::test_arc();
    // test_arc::test_demo();


    // test_mutex::test_mutex();
    // test_mutex::test_rw_lock();


    // test_cell::test_cell();
    // test_cell::test_ref_cell();
    // test_cell::test_cell2();


    // test_other_trait::test_from();
    // test_other_trait::test_into();


    // test_asref::test_asref();
    // test_asref::test_asmut();

    // test_borrow::test_borrow();
    // test_borrow::test_borrow_mut();
    // test_borrow::test_owned();

    // test_deref::test_deref();
    // test_deref::test_deref2();

    // test_cow::test_cow();
    // test_cow::test_cow3();

    // test_thread::test_thread();
    // test_thread::test_thread_msg_transmit();
    // test_thread::test_async_channel();

    // test_thread::test_sync_channel();

    // test_share_mem::test_static();
    // test_share_mem::test_box();

    // test_notify::test_notify();
    // test_notify::test_automic();
    // test_notify::test_mutex();
    // test_notify::test_mutex_notify();

    // test_parallel::test_parallel();

    // test_unsafe::test_unsafe();
    // test_unsafe::test_rw_static();
    // test_unsafe::test_invoke_unsafe();

    test_any::test_any();

}



