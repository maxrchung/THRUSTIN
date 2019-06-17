import React from 'react';
import Form from 'react-bootstrap/Form';
import Typeahead from 'react-bootstrap-typeahead';

class CommandBar extends React.Component {
    render() {
        return (
            <div className="mb-3 mr-3">
                <Form.Control ref={(input) => {this.input = input}} type="text" placeholder="Enter command..." />
            </div>
        );
    }
}

export default CommandBar;