import { Component } from '@angular/core';
import NavItem from '../model/NavItem';

@Component({
  selector: 'app-nav',
  templateUrl: './nav.component.pug',
  styleUrls: ['./nav.component.scss']
})
export class NavComponent {
  items = [
    new NavItem()
      .withActive(false)
      .withText("Users")
      .withLink("users")
  ];
}
