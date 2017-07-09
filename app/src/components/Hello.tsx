import * as React from "react";

export interface HelloProps { compiler: string; framework: string; }

// 'HelloProps' describes the shape of props.
// State is never set so we use the 'undefined' type.

// Add is is-active class next to navbar-menu to toggle for mobile
export class Hello extends React.Component<any, undefined> {
  render() {
    return (
      <div>
        <div className="columns">
          <div className="column">
            First column
          </div>
          <div className="column">
            Second column
          </div>
          <div className="column">
            Third column
          </div>
          <div className="column">
            Fourth column
          </div>
        </div>
      </div>
    )
  }
}
