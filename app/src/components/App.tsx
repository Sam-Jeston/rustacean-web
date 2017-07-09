import * as React from "react";
import {
  BrowserRouter as Router,
  Route,
  Link
} from 'react-router-dom'
import { Hello } from './Hello'
import { About } from './About'
const rustacean = require('../../public/rustacean.png')

// 'HelloProps' describes the shape of props.
// State is never set so we use the 'undefined' type.

// Add is is-active class next to navbar-menu to toggle for mobile

const inactiveClasses = "navbar-menu"
const activeClasses = "navbar-menu is-active"

export class App extends React.Component<any, {active: boolean}> {
  constructor(props: any) {
    super(props)
    this.state = { active: false }
  }

  public toggleIsActive () {
    this.setState({
      active: !this.state.active
    })
  }

  render() {
    return (
      <Router>
        <div>
          <nav className="navbar is-transparent">
            <div className="navbar-brand">
              <a className="navbar-item" href="https://techjeston.com">
                <img src={`dist/${rustacean}`} alt="Rustaceans on the Web" width="70" height="52" />
              </a>

              <div className="navbar-burger burger" data-target="navMenuExample" onClick={ () => this.toggleIsActive() }>
                <span></span>
                <span></span>
                <span></span>
              </div>
            </div>

            <div id="navMenuExample" className={this.state.active ? activeClasses : inactiveClasses}>
              <div className="navbar-start">
                <Link className="navbar-item" to="/">Home</Link>
                <Link className="navbar-item" to="/about">About</Link>
              </div>

              <div className="navbar-end">
                <a className="navbar-item" href="https://github.com/Sam-Jeston/rustacean-web" target="_blank">
                  Github
                </a>
              </div>
            </div>
          </nav>
          <Route exact path="/" component={Hello}/>
          <Route path="/about" component={About}/>
        </div>
      </Router>
    )
  }
}
