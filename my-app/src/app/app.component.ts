import { Component, OnInit } from '@angular/core';

import { RoomService } from './room-service.service';
import { RoomInfo } from './room-service.service';

import { Observable } from 'rxjs';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  roomInfo !: Observable<RoomInfo[]>;

  constructor(
    private roomService: RoomService
  ) { };

  ngOnInit(): void {
    this.roomInfo = this.roomService.getRooms();
  };

  title = "my_app";
}
