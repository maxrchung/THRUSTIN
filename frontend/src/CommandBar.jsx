import React from "react";
import { Typeahead } from "react-bootstrap-typeahead";

class CommandBar extends React.Component {
    state = {
		options: ["okay", ".help", ".name", ".login", ".register", ".color", ".join", ".list", ".make", ".play", ".THRUST", ".UNTHRUST", ".who", ".account", ".username", ".password", ".ban", ".unban", ".chieftain", ".unchieftain"],
	};

	render() {
		const {options} = this.state;
		return (
			<Typeahead
				autoFocus={true}
				id="commandBar"
				inputProps={{
					type: this.props.type
				}}
				onInputChange={this.props.onInputChange}
				options={options}
				placeholder="Enter command..."
				ref={typeahead => this.typeahead = typeahead}
				onKeyDown={this.props.onKeyDown}
			/>
		);
	}
}

export default CommandBar;