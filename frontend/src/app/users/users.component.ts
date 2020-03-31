import { Component, ViewChild } from '@angular/core';
import { environment } from './../../environments/environment';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-users',
  templateUrl: './users.component.pug',
  styleUrls: ['./users.component.scss']
})
export class UsersComponent {

  @ViewChild('result') result: any;
  @ViewChild('name') name: any;


  constructor(private http: HttpClient) {}

  async fetchUser() {
    const value = this.name.nativeElement.value;
    console.log(value);
    if (value.trim().length === 0) {
      return
    }

    const response = await
      this.http
        .get(environment.backendUrl + "/hello/name/" + value, { responseType: 'text' })
        .toPromise();
    this.result.nativeElement.innerHTML = response;
  }
}
