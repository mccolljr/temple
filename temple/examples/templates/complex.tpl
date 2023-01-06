I will now iterate my items:
{%- for (idx, i) in self.items.iter().enumerate() { %}
Item {{ idx }}: {{
    match i {
        Item::A(v) => v.as_dyn_display(),
        Item::B(v) => v.as_dyn_display(),
        Item::C(v) => v.as_dyn_display(),
    }
}}
{%- } -%}