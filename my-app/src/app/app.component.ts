import { Component, OnInit } from '@angular/core';

import { RoomService } from './room-service.service';
import { RoomInfo } from './room-service.service';
import {ApiService} from "./api.service";
import { Observable } from 'rxjs';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  roomInfo !: Observable<RoomInfo[]>;
data: any;
  constructor(
    private roomService: RoomService,
    private api: ApiService
  ) { };

  ngOnInit(): void {
    this.api.getData().subscribe(res=>{
      this.data=res;
    });
    this.roomInfo = this.roomService.getRooms();
  };

  title = "my_app";
}
