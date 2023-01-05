{%- let smoogity = 1; -%}

I am the main template: {{ self.name }}
I have sub-templates: {% for t in &self.sub_templates { %}
    {{ t }}
{%- } %}