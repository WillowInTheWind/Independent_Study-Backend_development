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
    location.reload();
  }
  hasnumber() {
    this.user.getuser().then(data => {
      data.subscribe(
        data=> {
          if (data.phone_number) {
            this.hasnum = true
          }
          else{ this.hasnum = true; }
        }
    )})
  }
  ngOnInit() {
    this.hasnumber();

  }
  constructor(private user: AuthorizationService) {
  }
}
