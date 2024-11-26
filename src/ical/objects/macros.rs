

macro_rules! ical_prop_name {
    ($param:ident) => {
        stringify!($param).to_uppercase().replace("_", "-")
    };
}

pub(crate) use ical_prop_name;

//OMG I LOVE macros
macro_rules! make_ical_comp_struct {
    (
        $(#[$struct_meta:meta])*
        $name:ident {
             $(
                $(#[$field_meta:meta])*
                $field:ident $type1:ident $type2:ident,
             )*
        }
    ) => {
        $(#[$struct_meta])*
        #[derive(Default)]
        pub struct $name {
            $(
                $(#[$field_meta])*
                pub $field: make_ical_comp_struct!(@field_type $type1 $type2),
            )*
        }

        impl $name {
            pub fn get_field_names() -> Vec<&'static str> {
                vec![$(stringify!($field)),*]
            }
        }
    };

    (@field_type Vec $type:ident) => {
        Vec<$type>
    };

    (@field_type Mul $type:ident) => {
        ICalMultiple<make_ical_comp_struct!(@ical_type $type)>
    };

    (@field_type Opt $type:ident) => {
        ICalOptional<make_ical_comp_struct!(@ical_type $type)>
    };

    (@ical_type String) => { ICalString };
    (@ical_type DateTime) => { ICalDateTime };
    (@ical_type Integer) => { ICalInteger };
    (@ical_type Duration) => { ICalDuration };
    (@ical_type $other:ident) => { $other };
}

pub(crate) use make_ical_comp_struct;
