import './App.css';
import EMPTY_CAPE from './empty_cape.png';
import 'bootstrap/dist/css/bootstrap.min.css';
import { Component } from 'react';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form'
import Col from 'react-bootstrap/Col'
import ReactSkinview3d from 'react-skinview3d'
const API = process.env.DD_API_URL || "http://127.0.0.1:3030";

class App extends Component {

  constructor(props) {
    super(props);
    this.state = {
      name: 'DiamondDagger590',
      busy: false
    }

    this.model = <ReactSkinview3d
      capeUrl={EMPTY_CAPE}
      skinUrl={API + "/profile?username=" + this.state.name}
      height="250"
      width="250"
      enableOrbitControls={false}
      onReady={(instance) => {
        this._modelInstance = instance;
        this._modelInstance.animations.add((player, time) => {
          player.rotation.y += .033;
        });
        this.diamonddaggify(null);
      }}

    />
    this.handleChange = this.handleChange.bind(this);
    this.handleKeyDown = this.handleKeyDown.bind(this);
    this.downloadSkin = this.downloadSkin.bind(this);
    this.uploadSkin = this.uploadSkin.bind(this);
    this.diamonddaggify = this.diamonddaggify.bind(this);
  }

  handleChange(e) {
    this.setState({ name: e.target.value, busy: this.state.busy });
  }

  handleKeyDown(e) {
    if (e.key === 'Enter') {
      this.diamonddaggify(e);
    }
  }

  diamonddaggify(e) {
    if (e !== null) {
      e.preventDefault();
    }

    this.setBusy(true);
    let skinUrl = API + "/profile?username=" + this.state.name;

    this._modelInstance.loadSkin(skinUrl).then(_ => this.setBusy(false))
    .catch(error => {
      alert("Failed to get skin from api, probably an invalid name!");
      this.setBusy(false);
    });
  }

  setBusy (busy) {
    this.setState({ name: this.state.name, busy: busy })
  }

  downloadSkin(e) {
    e.preventDefault();
    window.location.href = API + "/profile?username=" + this.state.name + "&download=true";
  }

  uploadSkin(e) {
    e.preventDefault();
    window.location.href = "https://www.minecraft.net/profile/skin/remote?url=" + API + "/profile?username=" + this.state.name + "?v302";
  }

  render() {

    return (
      <div className="App">
        <h1 style={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>DiamondDaggify your skin!</h1>
        <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '25vh' }}>
          {this.model}
        </div>
        <div>
          <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center' }}>
            <Form.Group>
              <Form.Row className="align-items-center">
                <Col xs="auto">
                  <Form.Control className="text-center" onKeyDown={this.handleKeyDown} onChange={this.handleChange} type="username" placeholder="Enter username" />
                </Col>
                <Col xs='auto'>
                  <Button variant="primary" onClick={this.diamonddaggify} disabled={this.state.busy}>DiamondDaggify!</Button>{' '}
                </Col>
              </Form.Row>
            </Form.Group>
          </div>
          <Button variant="secondary" onClick={this.downloadSkin}>Download</Button>{' '}
          <Button variant="success" onClick={this.uploadSkin}>Upload to minecraft.net</Button>{' '}
        </div>
      </div >
    );
  }
}

export default App;