import React from "react";
import { Typeahead } from "react-bootstrap-typeahead";

class CommandBar extends React.Component {
	render() {
		return (
			<Typeahead
				autoFocus={true}
				id="commandBar"
				inputProps={{
					type: this.props.type
				}}
				onInputChange={this.props.onInputChange}
				options={this.props.options}
				placeholder="Enter command..."
				ref={typeahead => this.typeahead = typeahead}
				onKeyDown={this.props.onKeyDown}
			/>
		);
	}
}

export default CommandBar;