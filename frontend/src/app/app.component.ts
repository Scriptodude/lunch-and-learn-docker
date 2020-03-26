import { Component } from '@angular/core';
import NavItem from './model/NavItem';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.pug',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'This is the frontend';
  content = 'This is the main page';
  items = [
    new NavItem()
      .withActive(false)
      .withText("Something")
      .withOnClick(this.setContent.bind(this, 'This is something')),
    new NavItem()
      .withActive(false)
      .withText("Else")
      .withOnClick(this.setContent.bind(this, 'This is something else'))
  ];

  public setContent(text: string) {
    if (text === null) {
      this.content = 'This is the main page';
      return;
    }

    this.content = text;
  }
}
