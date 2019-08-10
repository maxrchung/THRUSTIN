import React from 'react';
import { Typeahead } from 'react-bootstrap-typeahead';

class CommandBar extends React.Component {
    state = {
		options: ["Okay", "Epic"],
	};

	render() {
		const {options} = this.state;

		return (
			<Typeahead
				autoFocus={true}
				id="commandBar"
				onChange={(selected) => this.setState({selected})}
				onKeyDown={this.handleKeyDown}
				options={options}
				placeholder="Enter command..."
				ref={typeahead => this.typeahead = typeahead}
			/>
		);
	}
}

export default CommandBar;