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

            ///Includes 3.8.8.1 IANA Properties and 3.8.8.2 Non-Standard/X-Props
            pub unknown: Vec<ICalObject>,
        }

        impl ICSAble for $name {
            fn to_ics(&self, ics: &mut String) {
                serializer::begin(ics, $name::NAME);
                $(
                    self.$field.to_ics_with_name(&ical_prop_name!($field), ics);
                )*
                self.unknown.to_ics(ics);
                serializer::end(ics, $name::NAME);
            }
        }

        impl Parsable for $name {
            fn parse(lines: &mut IntoIter<ContentLine>, _: ContentLine) -> Result<Self, Box<dyn Error>> {
                let mut obj = Self::default();
                while let Some(line) = lines.next() {
                    if line.name == "END" {
                        if line.value == $name::NAME {
                            break
                        }
                        return Err(format!("Unexpected END in VCALENDAR. Found {}.", line.value).into())
                    }
                    $(
                        else if make_ical_comp_struct!(@parse_query line $field $type1 $type2) {
                            make_ical_comp_struct!(@parse_expr obj lines line $field $type1 $type2);
                        }
                    )*
                    else if line.name == "BEGIN" {
                        obj.unknown.push(ICalObject::parse(lines, line)?);
                    }
                    else {
                        obj.unknown.push(line.to_unknown_prop_obj())
                    }
                }

                Ok(obj)
            }
        }
    };

    (@field_type Children $type:ident) => {
        Vec<$type>
    };

    (@field_type Mul $type:ident) => {
        ICalMultiple<make_ical_comp_struct!(@ical_type $type)>
    };

    (@field_type Opt $type:ident) => {
        ICalOptional<make_ical_comp_struct!(@ical_type $type)>
    };

    (@parse_query $line:ident $field:ident Children $type:ident) => {
        $line.name == "BEGIN" && $line.value == $type::NAME
    };

    (@parse_query $line:ident $field:ident Mul $type:ident) => {
        $line.name == ical_prop_name!($field)
    };

    (@parse_query $line:ident $field:ident Opt $type:ident) => {
        $line.name == ical_prop_name!($field)
    };

    (@parse_expr $obj:ident $lines:ident $line:ident $field:ident Children $type:ident) => {
        $obj.$field.push($type::parse($lines, $line)?)
    };

    (@parse_expr $obj:ident $lines:ident $line:ident $field:ident Mul $type:ident) => {
        $obj.$field.add(&$line.value, $line.params)?
    };

    (@parse_expr $obj:ident $lines:ident $line:ident $field:ident Opt $type:ident) => {
        $obj.$field.set(&$line.value, $line.params)?
    };

    (@ical_type String) => { ICalString };
    (@ical_type DateTime) => { ICalDateTime };
    (@ical_type Integer) => { ICalInteger };
    (@ical_type Duration) => { ICalDuration };
    (@ical_type $other:ident) => { $other };
}

pub(crate) use make_ical_comp_struct;
