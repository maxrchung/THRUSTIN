import React from "react";
import { Typeahead } from "react-bootstrap-typeahead";

class CommandBar extends React.Component {
	render() {
		return (
			<Typeahead
				autoFocus={true}
				// id is required by Typeahead component
				id="commandBar" 
				inputProps={{
					type: this.props.type
				}}
				onInputChange={this.props.onInputChange}
				options={this.props.options}
				placeholder="Enter command..."
				ref={typeahead => this.typeahead = typeahead}
			/>
		);
	}
}

export default CommandBar;