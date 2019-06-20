import React from 'react';
import {Typeahead} from 'react-bootstrap-typeahead';

class CommandBar extends React.Component {
    state = {
        options: ["Okay", "Epic"],
    };
    
	render() {
		const {multiple, options} = this.state;

		return (
			<React.Fragment>
				<Typeahead
					id="commandBar"
					autoFocus={true}
					ref={(input) => {this.wrappedTypeahead = input}}
					options={options}
					placeholder="Enter command..."
				/>
			</React.Fragment>
		);
	}
}

export default CommandBar;