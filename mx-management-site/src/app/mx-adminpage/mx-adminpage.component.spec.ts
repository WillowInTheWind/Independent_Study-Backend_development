import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MxAdminpageComponent } from './mx-adminpage.component';

describe('MxAdminpageComponent', () => {
  let component: MxAdminpageComponent;
  let fixture: ComponentFixture<MxAdminpageComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MxAdminpageComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MxAdminpageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
