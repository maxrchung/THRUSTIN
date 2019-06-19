import React from 'react';
import Form from 'react-bootstrap/Form';
import {Typeahead} from 'react-bootstrap-typeahead';

class CommandBar extends React.Component {
    state = {
        options: ["Okay", "Epic"],
        multiple: false,
      };
    
      render() {
        const {multiple, options} = this.state;
    
        return (
          <React.Fragment>
            <Typeahead
				id="commandBar"
				ref={(input) => {this.input = input}}
                multiple={multiple}
                options={options}
                placeholder="Enter command..."
            />
          </React.Fragment>
        );
      }
    // render() {
    //     return (
    //         <React.Fragment>
    //             <Typeahead
    //                 id="retard"
    //                 ref={(input) => {this.input = input}}  
    //                 onChange={(selected) => {
    //                     console.log("Aight Bro U Poppin The Fuck Off My Man")
    //                 }}
    //                 options={
    //                     this.options
    //                 }>
    //             </Typeahead>
    //         </React.Fragment>
    //     );

    //     // return (
    //     //     <div className="mb-3 mr-3">
    //     //         <Form.Control ref={(input) => {this.input = input}} type="text" placeholder="Enter command..." />
    //     //     </div>
    //     // );
    // }
}

export default CommandBar;