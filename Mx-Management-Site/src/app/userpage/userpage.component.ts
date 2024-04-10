import { Component } from '@angular/core';
import {AuthorizationService} from "../authorization.service";
import {CommonModule} from "@angular/common";

@Component({
  selector: 'app-userpage',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './userpage.component.html',
  styleUrl: './userpage.component.css'
})
export class UserpageComponent {
protected validNum: Boolean = false;
  protected hasnum: Boolean = false;

  setnumber(number: string) {

    this.user.setNumber(number)
  }
  hasnumber() {
    this.user.getuser().then(data => {
      data.subscribe(
        data=> {
          if (!data) {
            this.hasnum = false
          }
          else{ this.hasnum = true; }
        }
    )})
  }
  constructor(private user: AuthorizationService) {
  }
}
