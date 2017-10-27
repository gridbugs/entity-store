#[derive(Debug, Clone, Copy)]
pub enum ComponentType {
    {% for _, component in components %}
        {{ component.name }} = {{ component.index }},
    {% endfor %}
}

#[derive(Debug, Clone)]
pub enum ComponentValue {
    {% for _, component in components %}
        {% if component.type %}
            {{ component.name }}({{ component.type }}),
        {% else %}
            {{ component.name }},
        {% endif %}
    {% endfor %}
}
