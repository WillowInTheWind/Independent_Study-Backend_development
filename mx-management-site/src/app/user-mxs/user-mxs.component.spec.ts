import { ComponentFixture, TestBed } from '@angular/core/testing';

import { UserMxsComponent } from './user-mxs.component';

describe('UserMxsComponent', () => {
  let component: UserMxsComponent;
  let fixture: ComponentFixture<UserMxsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [UserMxsComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(UserMxsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
