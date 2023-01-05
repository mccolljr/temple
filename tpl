{% for x in 0..100 { %}
{% self.internal_counter.set(self.internal_counter.get()+1); %}
    {{ self.internal_counter.get() }}
{% } %}