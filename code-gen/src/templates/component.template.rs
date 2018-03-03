#![allow(dead_code)]

enum_from_primitive! {
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    {% for _, component in components %}
        {{ component.name }} = {{ component.index }},
    {% endfor %}
}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentValue {
    {% for _, component in components %}
        {% if component.type %}
            {{ component.name }}({{ component.type }}),
        {% else %}
            {{ component.name }},
        {% endif %}
    {% endfor %}
}

pub enum ComponentRef<'a> {
    {% for _, component in components %}
        {% if component.type %}
            {{ component.name }}(&'a {{ component.type }}),
        {% else %}
            {{ component.name }},
        {% endif %}
    {% endfor %}
}

impl ComponentValue {
    pub fn typ(&self) -> ComponentType {
        match self {
            {% for _, component in components %}
                {% if component.type %}
                    &ComponentValue::{{ component.name }}(_) => ComponentType::{{ component.name }},
                {% else %}
                    &ComponentValue::{{ component.name }} => ComponentType::{{ component.name }},
                {% endif %}
            {% endfor %}
        }
    }
}

impl<'a> ComponentRef<'a> {
    pub fn typ(&self) -> ComponentType {
        match self {
            {% for _, component in components %}
                {% if component.type %}
                    &ComponentRef::{{ component.name }}(_) => ComponentType::{{ component.name }},
                {% else %}
                    &ComponentRef::{{ component.name }} => ComponentType::{{ component.name }},
                {% endif %}
            {% endfor %}
        }
    }
}
