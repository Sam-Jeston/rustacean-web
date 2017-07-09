import * as React from "react";

export interface HelloProps { compiler: string; framework: string; }

// 'HelloProps' describes the shape of props.
// State is never set so we use the 'undefined' type.

// Add is is-active class next to navbar-menu to toggle for mobile
export class Hello extends React.Component<HelloProps, undefined> {
  render() {
    return (
      <div>
        <nav className="navbar is-transparent">
          <div className="navbar-brand">
            <a className="navbar-item" href="http://bulma.io">
              <img src="http://bulma.io/images/bulma-logo.png" alt="Bulma: a modern CSS framework based on Flexbox" width="112" height="28" />
            </a>

            <div className="navbar-burger burger" data-target="navMenuExample">
              <span></span>
              <span></span>
              <span></span>
            </div>
          </div>

          <div id="navMenuExample" className="navbar-menu">
            <div className="navbar-start">
              <a className="navbar-item " href="http://bulma.io/">
                Home
              </a>
            </div>

            <div className="navbar-end">
              <a className="navbar-item" href="https://github.com/Sam-Jeston/rustacean-web" target="_blank">
                Github
              </a>
            </div>
          </div>
        </nav>
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
