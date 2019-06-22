import React from 'react';
import {Typeahead} from 'react-bootstrap-typeahead';

class CommandBar extends React.Component {
    state = {
		options: ["Okay", "Epic"],
	};

	render() {
		const {options} = this.state;

		return (
			<React.Fragment>
				<Typeahead
					id="commandBar"
					autoFocus={true}
					ref={(input) => {this.wrappedTypeahead = input}}
					options={options}
					onChange={(selected) => this.setState({selected})}
					onKeyDown={this.handleKeyDown}
					placeholder="Enter command..."
				/>
			</React.Fragment>
		);
	}
}

export default CommandBar;